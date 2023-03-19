/*
 * @Date: 2023-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-19
 * @FilePath: /algorithm/rust/1625_find_lex_smallest_string/find_lex_smallest_string.rs
 */

pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    use std::collections::{HashSet, VecDeque};

    let chars = s.bytes().collect::<Vec<u8>>();
    let n = chars.len();
    let mut ans = s.clone();
    let mut q = VecDeque::<String>::new();
    q.push_back(ans.clone());
    let mut visited = HashSet::<String>::new();
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        if ans > cur {
            ans = cur.clone();
        }
        let mut ca = cur.bytes().collect::<Vec<u8>>();
        for i in (1..n).step_by(2) {
            ca[i] = (ca[i] - b'0' + a as u8) % 10 + b'0';
        }
        let added = String::from_utf8(ca).unwrap();
        if visited.insert(added.clone()) {
            q.push_back(added);
        }
        let mut rotated = String::new();
        rotated.push_str(&cur[n - b as usize..]);
        rotated.push_str(&cur[..n - b as usize]);
        if visited.insert(rotated.clone()) {
            q.push_back(rotated);
        }
    }
    ans
}

fn main() {
    assert_eq!(
        find_lex_smallest_string("5525".to_owned(), 9, 2),
        "2050".to_owned()
    );

    assert_eq!(
        find_lex_smallest_string("5525".to_owned(), 9, 2),
        "2050".to_owned()
    );
    assert_eq!(
        find_lex_smallest_string("43987654".to_owned(), 7, 3),
        "00553311".to_owned()
    );
    assert_eq!(
        find_lex_smallest_string("0011".to_owned(), 4, 2),
        "0011".to_owned()
    );
}
