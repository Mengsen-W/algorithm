/*
 * @Date: 2021-11-10 00:52:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-10 01:22:10
 */

struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let n = time_series.len();
        let (mut sum, mut l, mut r) = (0, 0, 0);
        for i in 0..n {
            if time_series[i] > r {
                sum += r - l;
                l = time_series[i];
                r = time_series[i] + duration;
            } else {
                r = time_series[i] + duration;
            }
        }
        sum += r - l;
        sum
    }
}

fn main() {
    assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
}
