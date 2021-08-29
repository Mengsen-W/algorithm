/*
 * @Date: 2021-08-29 16:57:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-29 17:23:15
 */

struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = arr.len();
        for i in 0..n {
            let left_count = i;
            let right_count = n - i - 1;
            let left_odd = (left_count + 1) / 2;
            let right_odd = (right_count + 1) / 2;
            let left_even = left_count / 2 + 1;
            let right_even = right_count / 2 + 1;
            sum += arr[i] * (left_odd * right_odd + left_even * right_even) as i32;
        }
        sum
    }
}

fn main() {
    {
        let arr = vec![1, 4, 2, 5, 3];
        let ans = 58;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), ans);
    }
    {
        let arr = vec![1, 2];
        let ans = 3;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), ans);
    }
    {
        let arr = vec![10, 11, 12];
        let ans = 66;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), ans);
    }
}
