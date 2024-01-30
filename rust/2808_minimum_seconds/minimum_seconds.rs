/*
 * @Date: 2024-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-30
 * @FilePath: /algorithm/rust/2808_minimum_seconds/minimum_seconds.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let nlen = nums.len();
        let mut positions = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            positions.entry(n).or_insert(vec![]).push(i);
        }

        positions
            .into_values()
            .map(|pos| {
                let len = pos.len();
                let mut pre = pos[0];
                pos.into_iter()
                    .cycle()
                    .take(len + 1)
                    .skip(1)
                    .map(|p| {
                        let pp = pre;
                        pre = p;
                        (p + nlen - pp) % nlen
                    })
                    .map(|p| if p == 0 { nlen } else { p })
                    .max()
                    .unwrap()
            })
            .min()
            .map(|dis| dis / 2)
            .unwrap() as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1, 2], 1),
        (vec![2, 1, 3, 3, 2], 2),
        (vec![5, 5, 5, 5], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_seconds(nums), ans);
    }
}
