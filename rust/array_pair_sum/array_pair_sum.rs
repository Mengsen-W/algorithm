/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 22:22:23
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:31:16
 */

fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums.into_iter().step_by(2).sum()
}

fn main() {
    let mut nums: Vec<i32>;
    nums = vec![1, 4, 3, 2];
    assert_eq!(4, array_pair_sum(nums));
    nums = vec![6, 2, 6, 5, 1, 2];
    assert_eq!(9, array_pair_sum(nums));
}
