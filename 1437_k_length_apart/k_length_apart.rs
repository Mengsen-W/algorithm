/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:36:00
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:40:53
 */

fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut dst = i32::MAX as usize;
    for i in (0..nums.len()).rev() {
        if nums[i] == 1 {
            if dst - i <= k {
                return false;
            } else {
                dst = i
            }
        }
    }
    true
}

fn main() {
    let mut nums: Vec<i32>;
    nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
    println!("{}", k_length_apart(nums, 2));
    nums = vec![1, 0, 0, 1, 0, 1];
    println!("{}", k_length_apart(nums, 2));
    nums = vec![1, 1, 1, 1, 1];
    println!("{}", k_length_apart(nums, 0));
    nums = vec![0, 1, 0, 1];
    println!("{}", k_length_apart(nums, 0));
}
