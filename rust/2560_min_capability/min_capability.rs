/*
 * @Date: 2023-09-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-19
 * @FilePath: /algorithm/rust/2560_min_capability/min_capability.rs
 */

struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut lower = *nums.iter().min().unwrap();
        let mut upper = *nums.iter().max().unwrap();

        while lower <= upper {
            let middle = (lower + upper) / 2;
            let mut count = 0;
            let mut visited = false;

            for &x in nums.iter() {
                if x <= middle && !visited {
                    count += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }
            if count >= k {
                upper = middle - 1;
            } else {
                lower = middle + 1;
            }
        }
        lower
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 5, 9], 2, 5), (vec![2, 7, 9, 3, 1], 2, 2)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::min_capability(nums, k), ans);
    }
}
