/*
 * @Date: 2024-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-29
 * @FilePath: /algorithm/rust/2908_minimum_sum/minimum_sum.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 1000;
        let mut mn = 1000;
        let mut left = vec![0; n];
        for i in 1..n {
            mn = nums[i - 1].min(mn);
            left[i] = mn;
        }

        let mut right = nums[n - 1];
        for i in (1..n - 1).rev() {
            if left[i] < nums[i] && nums[i] > right {
                res = res.min(left[i] + nums[i] + right);
            }
            right = right.min(nums[i]);
        }

        if res < 1000 {
            return res;
        }
        return -1;
    }
}

fn main() {
    let tests = vec![
        (vec![8, 6, 1, 5, 3], 9),
        (vec![5, 4, 8, 7, 10, 2], 13),
        (vec![6, 5, 4, 3, 4, 5], -1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_sum(nums), ans);
    }
}
