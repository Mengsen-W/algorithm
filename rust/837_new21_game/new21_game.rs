struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let mut f = vec![0.0; n + 1];
        let mut s = 0.0;
        for i in (0..=n).rev() {
            f[i] = if i >= k { 1.0 } else { s / max_pts as f64 };
            // 当前循环计算的是 f[i+1] + ... + f[i+maxPts]
            // 下个循环计算的是 f[i] + ... + f[i+maxPts-1]，多了 f[i]，少了 f[i+maxPts]
            s += f[i];
            if i + max_pts <= n {
                s -= f[i + max_pts];
            }
        }
        f[0]
    }
}

fn main() {
    let tests = vec![
        (10, 1, 10, 1.0000),
        (6, 1, 10, 0.60000),
        (21, 17, 10, 0.73278),
    ];

    for (n, k, max_pts, ans) in tests {
        assert_eq!(Solution::new21_game(n, k, max_pts), ans);
    }
}
