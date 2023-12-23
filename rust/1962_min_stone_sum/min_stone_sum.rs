/*
 * @Date: 2023-12-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-23
 * @FilePath: /algorithm/rust/1962_min_stone_sum/min_stone_sum.rs
 */

struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut h = BinaryHeap::from(piles); // 原地堆化（最大堆）
        for _ in 0..k {
            let top = h.pop().unwrap();
            h.push((top + 1) / 2);
            if *h.peek().unwrap() == 0 {
                break;
            }
        }
        h.iter().sum()
    }
}

fn main() {
    let tests = vec![(vec![5, 4, 9], 2, 12), (vec![4, 3, 6, 7], 3, 12)];

    for (piles, k, ans) in tests {
        assert_eq!(Solution::min_stone_sum(piles, k), ans);
    }
}
