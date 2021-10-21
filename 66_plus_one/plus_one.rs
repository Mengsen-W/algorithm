/*
 * @Date: 2021-10-21 01:13:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-21 01:50:29
 */

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        for i in (0..=n - 1).rev() {
            digits[i] += 1;
            if digits[i] / 10 == 0 {
                return digits;
            }
            digits[i] = digits[i] % 10
        }
        digits.insert(0, 1);
        digits
    }
}

fn main() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
    assert_eq!(Solution::plus_one(vec![1, 9, 9]), vec![2, 0, 0]);
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
}
