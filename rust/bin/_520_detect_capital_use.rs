/*
 * @Date: 2021-11-13 01:15:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-13 01:28:51
 */

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut cnt = 0;
        let len = word.len();

        for c in word.chars() {
            if c.is_uppercase() {
                cnt += 1;
            }
        }
        let ch = word.chars().next().unwrap();
        if cnt == len || cnt == 0 || (cnt == 1 && ch.is_uppercase()) {
            return true;
        }
        false
    }
}

fn main() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
}
