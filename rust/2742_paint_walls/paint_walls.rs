struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        use std::cmp::min;
        let n = cost.len();
        let mut f = vec![i32::MAX / 2; n * 2 + 1];
        f[n] = 0;
        for i in 0..n {
            let mut g = vec![i32::MAX / 2; n * 2 + 1];
            for j in 0..=n * 2 {
                // 付费
                g[min(j + time[i] as usize, n * 2)] =
                    min(g[min(j + time[i] as usize, n * 2)], f[j] + cost[i]);
                // 免费
                if j > 0 {
                    g[j - 1] = min(g[j - 1], f[j]);
                }
            }
            f = g;
        }
        *f.iter().skip(n).min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 2], vec![1, 2, 3, 2], 3),
        (vec![2, 3, 4, 2], vec![1, 1, 1, 1], 4),
    ];

    for (cost, time, ans) in tests {
        assert_eq!(Solution::paint_walls(cost, time), ans);
    }
}
