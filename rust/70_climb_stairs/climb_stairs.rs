/*
 * @Date: 2023-12-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-10
 * @FilePath: /algorithm/rust/70_climb_stairs/climb_stairs.rs
 */

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as f64;
        let sqrt5 = 5f64.sqrt();
        ((((1.0 + sqrt5) / 2.0).powf(n + 1.0) - ((1.0 - sqrt5) / 2.0).powf(n + 1.0)) / sqrt5)
            .round() as i32
    }
}

fn main() {
    let tests = vec![(2, 2), (3, 3)];

    for (n, ans) in tests {
        assert_eq!(Solution::climb_stairs(n), ans);
    }
}
