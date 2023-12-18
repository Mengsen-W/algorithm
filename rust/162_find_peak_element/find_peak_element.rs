/*
 * @Date: 2021-09-15 08:49:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-15 09:07:03
 */

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let get = |i: i32| -> (i32, i32) {
            if i == -1 || i == n {
                (0, 0)
            } else {
                (1, nums[i as usize])
            }
        };

        let (mut left, mut right, mut ans) = (0 as i32, n as i32 - 1, -1 as i32);
        while left <= right {
            let mid = (left + right) / 2;
            if get(mid - 1) < get(mid) && get(mid) > get(mid + 1) {
                ans = mid;
                break;
            }

            if get(mid) < get(mid + 1) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 1], 2), (vec![1, 2, 1, 3, 5, 6, 4], 5)];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_peak_element(nums), ans);
    }
}
