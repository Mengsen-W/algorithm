/*
 * @Date: 2024-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-13
 * @FilePath: /algorithm/rust/2864_maximum_odd_binary_number/maximum_odd_binary_number.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let cnt1 = s.chars().filter(|&c| c == '1').count();
        "1".repeat(cnt1 - 1) + &"0".repeat(s.len() - cnt1) + "1"
    }
}

fn main() {
    let tests = vec![("010", "001"), ("0101", "1001")];

    for (s, ans) in tests {
        assert_eq!(Solution::maximum_odd_binary_number(s.to_string()), ans);
    }
}
