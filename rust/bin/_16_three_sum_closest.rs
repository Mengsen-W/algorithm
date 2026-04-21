/*
 * @Date: 2023-07-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-10
 * @FilePath: /algorithm/rust/16_three_sum_closest/three_sum_closest.rs
 */

struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 1e7 as i32;
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target;
                }
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                if sum > target {
                    k -= 1;
                    while k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum < target {
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let test = vec![(vec![-1, 2, 1, -4], 1, 2), (vec![0, 0, 0], 1, 0)];
    for (nums, target, ans) in test {
        assert_eq!(Solution::three_sum_closest(nums, target), ans);
    }
}
