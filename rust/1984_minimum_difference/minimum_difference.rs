/*
 * @Date: 2022-02-11 00:14:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-11 00:28:47
 * @FilePath: /algorithm/1984_minimum_difference/minimum_difference.rs
 */

pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    nums.sort();
    nums.iter()
        .enumerate()
        .fold(nums[k - 1] - nums[0], |ret, (i, v)| {
            if i >= k {
                ret.min(v - nums[i - k + 1])
            } else {
                ret
            }
        })
}

fn main() {
    assert_eq!(minimum_difference(vec![90], 1), 0);
    assert_eq!(minimum_difference(vec![9, 4, 7, 1], 2), 2);
}
