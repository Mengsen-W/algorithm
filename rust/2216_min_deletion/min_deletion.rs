/*
 * @Date: 2023-11-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-21
 * @FilePath: /algorithm/rust/2216_min_deletion/min_deletion.rs
 */

struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && nums[j] == nums[i] {
                ans += 1;
                j += 1;
            }
            i = j + 1;
        }
        ans += (n - ans) % 2;
        ans as i32
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 2, 3, 5], 1), (vec![1, 1, 2, 2, 3, 3], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::min_deletion(nums), ans);
    }
}
