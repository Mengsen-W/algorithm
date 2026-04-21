/*
 * @Date: 2024-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-24
 * @FilePath: /algorithm/rust/1673_most_competitive/most_competitive.rs
 */

struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let n = nums.len();
        for i in 0..n {
            while res.len() > 0 && (n - i + res.len()) as i32 > k && *res.last().unwrap() > nums[i]
            {
                res.pop();
            }
            res.push(nums[i]);
        }
        res.truncate(k as usize);
        res
    }
}

fn main() {
    let tests = vec![
        (vec![3, 5, 2, 6], 2, vec![2, 6]),
        (vec![2, 4, 3, 3, 5, 4, 9, 6], 4, vec![2, 3, 3, 4]),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::most_competitive(nums, k), ans);
    }
}
