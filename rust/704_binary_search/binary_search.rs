/*
 * @Date: 2021-09-07 17:26:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:37:01
 */

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len() as i32 - 1);
        while low <= high {
            let mid = (high - low) / 2 + low;
            let num = nums[mid as usize];
            if num == target {
                return mid;
            } else if num > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return -1;
    }
}

fn main() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(Solution::search(vec![5], -5), -1);
}
