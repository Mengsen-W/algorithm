/*
 * @Date: 2022-02-26 01:33:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-26 02:07:44
 * @FilePath: /algorithm/2016_maximum_difference/maximum_difference.rs
 */

pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((-1, i32::MAX), |(mut ans, mut cur_min), num| {
            if cur_min < *num {
                ans = ans.max(*num - cur_min);
            }
            cur_min = cur_min.min(*num);
            (ans, cur_min)
        })
        .0
}

fn main() {
    assert_eq!(maximum_difference(vec![7, 1, 5, 4]), 4);
    assert_eq!(maximum_difference(vec![9, 4, 3, 2]), -1);
    assert_eq!(maximum_difference(vec![1, 5, 2, 10]), 9);
}
