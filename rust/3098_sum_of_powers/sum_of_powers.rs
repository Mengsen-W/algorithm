struct Solution;

impl Solution {
    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const INF: i32 = 0x3f3f3f3f;
        let mut nums = nums.clone();
        nums.sort();
        let n = nums.len();
        let mut vals: Vec<i32> = vec![];

        for i in 0..n {
            for j in 0..i {
                vals.push(nums[i] - nums[j]);
            }
        }
        vals.push(INF);
        vals.sort();
        vals.dedup();

        let m = vals.len();
        let mut d = vec![vec![vec![0; m]; k as usize + 1]; n];
        let mut border = vec![vec![0; k as usize + 1]; n];
        let mut sum = vec![vec![0; m]; k as usize + 1];
        let mut suf = vec![vec![0; k as usize + 1]; n];

        for i in 0..n {
            for j in 0..i {
                let pos = vals
                    .binary_search(&(&nums[i] - &nums[j]))
                    .unwrap_or_else(|e| e);
                for p in 1..=k {
                    let p = p as usize;
                    while border[j][p] < pos {
                        sum[p][border[j][p]] = (sum[p][border[j][p]] - suf[j][p] + MOD) % MOD;
                        sum[p][border[j][p]] =
                            (sum[p][border[j][p]] + d[j][p][border[j][p]] as i64) % MOD;
                        suf[j][p] = (suf[j][p] - d[j][p][border[j][p]] as i64 + MOD) % MOD;
                        border[j][p] += 1;
                        sum[p][border[j][p]] = (sum[p][border[j][p]] + suf[j][p]) % MOD;
                    }
                }
            }

            d[i][1][m - 1] = 1;
            for p in 2..=k {
                let p = p as usize;
                for v in 0..m {
                    d[i][p as usize][v] = sum[p - 1][v] as usize;
                }
            }
            for p in 1..=k {
                let p = p as usize;
                for v in 0..m {
                    suf[i][p] = (suf[i][p] + d[i][p][v] as i64) % MOD;
                }
                sum[p][0] = (sum[p][0] + suf[i][p]) % MOD;
            }
        }

        let mut res = 0;
        for i in 0..n {
            for v in 0..m {
                res = (res + vals[v] as i64 * d[i][k as usize][v] as i64 % MOD) % MOD;
            }
        }
        res as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4], 3, 4),
        (vec![2, 2], 2, 0),
        (vec![4, 3, -1], 2, 10),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::sum_of_powers(nums, k), ans);
    }
}
