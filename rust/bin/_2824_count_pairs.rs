/*
 * @Date: 2023-11-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-24
 * @FilePath: /algorithm/rust/2824_count_pairs/count_pairs.rs
 */

struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let (mut j, mut k, mut ret) = (0, nums.len(), 0);
        loop {
            k -= 1;
            let a = target - nums[k];
            j += nums[j..k].partition_point(|&x| x < a);
            if j == k {
                break;
            }
            ret += j;
        }
        ret += k * (k + 1) / 2;
        ret as i32
    }
}

fn main() {
    let tests = vec![
        (vec![-1, 1, 2, 3, 1], 2, 3),
        (vec![-6, 2, 5, -2, -7, -1, 3], -2, 10),
    ];

    for (nums, target, ans) in tests {
        assert_eq!(Solution::count_pairs(nums, target), ans);
    }
}
