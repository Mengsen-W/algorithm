/*
 * @Date: 2023-12-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-13
 * @FilePath: /algorithm/rust/2697_make_smallest_palindrome/make_smallest_palindrome.rs
 */

struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut cs: Vec<char> = s.chars().collect();
        let n = cs.len();
        for i in 0..n / 2 {
            let j = n - 1 - i;
            cs[i] = std::cmp::min(cs[i], cs[j]);
            cs[j] = cs[i];
        }
        cs.into_iter().collect()
    }
}

fn main() {
    let tests = vec![("egcfe", "efcfe"), ("abcd", "abba"), ("seven", "neven")];

    for (s, ans) in tests {
        assert_eq!(
            Solution::make_smallest_palindrome(s.to_string()),
            ans.to_string()
        );
    }
}
