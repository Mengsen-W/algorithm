/*
 * @Date: 2022-05-21 21:41:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-21 21:51:08
 * @FilePath: /algorithm/961_repeated_n_times/repeated_n_times.rs
 */

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    use rand::{thread_rng, Rng};
    let n = nums.len();
    loop {
        let (x, y) = (thread_rng().gen_range(0, n), thread_rng().gen_range(0, n));
        if x != y && nums[x] == nums[y] {
            return nums[x];
        }
    }
}

fn main() {}