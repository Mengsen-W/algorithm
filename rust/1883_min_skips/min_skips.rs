/*
 * @Date: 2024-04-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-19
 * @FilePath: /algorithm/rust/1883_min_skips/min_skips.rs
 */

struct Solution;

impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        // 可忽略误差
        let eps = 1e-7;
        let n = dist.len();
        let mut f = vec![vec![f64::INFINITY; n + 1]; n + 1];
        f[0][0] = 0.0;
        for i in 1..=n {
            for j in 0..=i {
                if j != i {
                    f[i][j] = f[i][j]
                        .min((f[i - 1][j] + (dist[i - 1] as f64) / (speed as f64) - eps).ceil());
                }
                if j != 0 {
                    f[i][j] = f[i][j].min(f[i - 1][j - 1] + (dist[i - 1] as f64) / (speed as f64));
                }
            }
        }

        for j in 0..=n {
            if f[n][j] < (hours_before as f64) + eps {
                return j as i32;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2], 4, 2, 1),
        (vec![7, 3, 5, 5], 2, 10, 2),
        (vec![7, 3, 5, 5], 1, 10, -1),
    ];

    for (dist, speed, hours_before, ans) in tests {
        assert_eq!(Solution::min_skips(dist, speed, hours_before), ans);
    }
}
