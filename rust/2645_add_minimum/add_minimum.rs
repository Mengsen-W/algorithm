/*
 * @Date: 2024-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-11
 * @FilePath: /algorithm/rust/2645_add_minimum/add_minimum.rs
 */

struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let n = word.len();
        let mut cnt = 1;
        let word = word.as_bytes();
        for i in 1..n {
            if word[i] <= word[i - 1] {
                cnt += 1;
            }
        }
        cnt * 3 - n as i32
    }
}

fn main() {
    let tests = vec![("b", 2), ("aaa", 6), ("abc", 0)];

    for (word, ans) in tests {
        assert_eq!(Solution::add_minimum(word.to_string()), ans);
    }
}
