/*
 * @Date: 2023-08-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-19
 * @FilePath: /algorithm/rust/2235_sum/sum.rs
 */

struct Solution;
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

fn main() {
    let tests = vec![(12, 5, 17), (-10, 4, -6)];

    for (num1, num2, result) in tests {
        assert_eq!(Solution::sum(num1, num2), result);
    }
}
