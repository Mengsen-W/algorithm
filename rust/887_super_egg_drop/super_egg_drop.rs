struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        use std::cmp::max;
        let mut dp = vec![0; (n + 1) as usize];
        for i in 0..=n {
            dp[i as usize] = i;
        }

        for _ in 2..=k {
            let mut dp2 = vec![0; (n + 1) as usize];
            let mut x = 1;
            dp2[0] = 0;
            for m in 1..=n {
                while x < m
                    && max(dp[(x - 1) as usize], dp2[(m - x) as usize])
                        >= max(dp[x as usize], dp2[(m - x - 1) as usize])
                {
                    x += 1;
                }
                dp2[m as usize] = 1 + max(dp[(x - 1) as usize], dp2[(m - x) as usize]);
            }
            dp.copy_from_slice(&dp2);
        }
        dp[n as usize]
    }
}

fn main() {
    let tests = vec![(1, 2, 2), (2, 6, 3), (3, 14, 4)];

    for (k, n, ans) in tests {
        assert_eq!(Solution::super_egg_drop(k, n), ans);
    }
}
