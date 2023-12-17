/*
 * @Date: 2023-12-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-17
 * @FilePath: /algorithm/rust/746_min_cost_climbing_stairs/min_cost_climbing_stairs.rs
 */

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut f, mut g) = (0, 0);
        for i in 2..=cost.len() {
            let gg = std::cmp::min(f + cost[i - 2], g + cost[i - 1]);
            f = g;
            g = gg;
        }
        g
    }
}

fn main() {
    let tests = vec![
        (vec![10, 15, 20], 15),
        (vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6),
    ];

    for (cost, ans) in tests {
        assert_eq!(Solution::min_cost_climbing_stairs(cost), ans);
    }
}
