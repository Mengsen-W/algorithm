/*
 * @Date: 2021-05-26 09:53:23
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-26 10:54:06
 */

fn reverse_parentheses(s: String) -> String {
    use std::collections::VecDeque;
    enum StackElement {
        Char(char),
        Deque(VecDeque<char>),
    }
    let mut stack = VecDeque::new();
    s.chars().fold(false, |acc, cur| match cur {
        ')' => {
            let mut d = VecDeque::new();
            loop {
                match stack.pop_back().unwrap() {
                    StackElement::Char('(') => {
                        if d.len() > 0 {
                            stack.push_back(StackElement::Deque(d))
                        }

                        break;
                    }
                    StackElement::Deque(v) => {
                        if !acc {
                            v.iter().rev().for_each(|e| d.push_front(*e))
                        } else {
                            d.append(&mut v.clone());
                        }
                    }
                    StackElement::Char(e) => {
                        if !acc {
                            d.push_front(e);
                        } else {
                            d.push_back(e);
                        }
                    }
                };
            }
            !acc
        }
        _ => {
            stack.push_back(StackElement::Char(cur));
            match cur {
                '(' => !acc,
                _ => acc,
            }
        }
    });

    stack.iter().fold(String::from(""), |acc, cur| match cur {
        StackElement::Deque(d) => [acc, d.iter().collect::<String>()].concat(),
        StackElement::Char(e) => [acc, e.to_string()].concat(),
    })
}

fn main() {
    assert_eq!(
        reverse_parentheses(String::from("(abcd)")),
        "dcba".to_string()
    );
    assert_eq!(
        reverse_parentheses(String::from("(u(love)i)")),
        "iloveu".to_string()
    );
    assert_eq!(
        reverse_parentheses(String::from("(ed(et(oc))el)")),
        "leetcode".to_string()
    );
    assert_eq!(
        reverse_parentheses(String::from("a(bcdefghijkl(mno)p)q")),
        "apmnolkjihgfedcbq".to_string()
    );
}
