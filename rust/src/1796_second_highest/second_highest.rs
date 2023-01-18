/*
 * @Date: 2022-12-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-03
 * @FilePath: /algorithm/1796_second_highest/second_highest.rs
 */

pub fn second_highest(s: String) -> i32 {
    use std::collections::HashSet;
    let mut vec = Vec::new();
    let mut hs = HashSet::new();
    for c in s.chars() {
        if c.is_digit(10) {
            let n = c.to_digit(10).unwrap();
            if !hs.contains(&n) {
                vec.push(n);
                hs.insert(n);
            }
        }
    }
    if vec.len() < 2 {
        return -1;
    }
    vec.sort();
    vec[vec.len() - 2] as i32
}

fn main() {
    assert_eq!(second_highest(String::from("dfa12321afd")), 2);
    assert_eq!(second_highest(String::from("abc1111")), -1);
}
