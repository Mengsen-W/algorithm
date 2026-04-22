/*
 * @Date: 2021-04-30 09:55:36
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-15
 */

struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        nums.iter().for_each(|num| {
            b = !a & (b ^ num);
            a = !b & (a ^ num);
        });
        b
    }
}

fn main() {
    let tests = vec![(vec![2, 2, 3, 2], 3), (vec![0, 1, 0, 1, 0, 1, 99], 99)];

    for (nums, ans) in tests {
        assert_eq!(Solution::single_number(nums), ans);
    }
}
