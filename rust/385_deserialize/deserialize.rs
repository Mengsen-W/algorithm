/*
 * @Date: 2022-04-15 09:32:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-15 09:41:17
 * @FilePath: /algorithm/385_deserialize/deserialize.rs
 */

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn deserialize(s: String) -> NestedInteger {
    use crate::nested_integer::NestedInteger::List;
    use std::str::FromStr;
    let chars = s.chars().collect::<Vec<char>>();
    let mut stack = vec![];
    let mut str = String::new();
    for c in chars {
        if c == '[' {
            stack.push(NestedInteger::List(vec![]))
        } else if '-' == c || c.is_digit(10) {
            if stack.is_empty() {
                return NestedInteger::Int(i32::from_str(&s).unwrap());
            } else {
                str.push(c);
            }
        } else if ',' == c {
            if !str.is_empty() {
                if let Some(v) = stack.last_mut() {
                    if let List(n) = v {
                        n.push(NestedInteger::Int(i32::from_str(&str).unwrap()));
                    }
                }
                str.truncate(0);
            }
        } else {
            if !str.is_empty() {
                if let Some(v) = stack.last_mut() {
                    if let List(n) = v {
                        n.push(NestedInteger::Int(i32::from_str(&str).unwrap()));
                    }
                }
                str.truncate(0);
            }
            let n = stack.pop().unwrap();
            if stack.is_empty() {
                return n;
            } else if let Some(v) = stack.last_mut() {
                if let List(nst) = v {
                    nst.push(n);
                }
            }
        }
    }
    NestedInteger::Int(-1)
}
