/*
 * @Date: 2021-08-28 14:51:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-28 15:08:46
 */

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |s, i| {
                *s += *i;
                Some(*s)
            })
            .collect()
    }
}

fn main() {
    {
        let nums = vec![1, 2, 3, 4];
        let ans = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(nums), ans);
    }
    {
        let nums = vec![1, 1, 1, 1, 1];
        let ans = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(nums), ans);
    }
    {
        let nums = vec![3, 1, 2, 10, 1];
        let ans = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(nums), ans);
    }
}
