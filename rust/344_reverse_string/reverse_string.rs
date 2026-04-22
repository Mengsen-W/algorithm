/*
 * @Date: 2023-08-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-07
 * @FilePath: /algorithm/rust/344_reverse_string/reverse_string.rs
 */

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        let mut c;
        for i in 0..len / 2 {
            c = s[i];
            s[i] = s[len - i - 1];
            s[len - i - 1] = c;
        }
    }
}

fn main() {
    let tests = vec![
        (vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h']),
        (
            vec!['H', 'a', 'n', 'n', 'a', 'h'],
            vec!['h', 'a', 'n', 'n', 'a', 'H'],
        ),
    ];

    for (mut s, expect) in tests {
        Solution::reverse_string(&mut s);
        assert_eq!(s, expect);
    }
}
