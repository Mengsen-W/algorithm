struct Solution;

use std::sync::OnceLock;

static POW10: OnceLock<Vec<i64>> = OnceLock::new();
const MOD: i64 = 1_000_000_007;

fn get_pow10() -> &'static Vec<i64> {
    POW10.get_or_init(|| {
        let mut p = vec![1i64; 100005];
        for i in 1..100005 {
            p[i] = (p[i - 1] * 10) % MOD;
        }
        p
    })
}

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let pow10 = get_pow10();
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut sum = vec![0i32; n + 1];
        let mut x = vec![0i64; n + 1];
        let mut cnt = vec![0i32; n + 1];
        for i in 0..n {
            let d = (s_bytes[i] - b'0') as i32;
            sum[i + 1] = sum[i] + d;
            x[i + 1] = if d > 0 {
                (x[i] * 10 + d as i64) % MOD
            } else {
                x[i]
            };
            cnt[i + 1] = cnt[i] + if d > 0 { 1 } else { 0 };
        }
        let m = queries.len();
        let mut res = vec![0i32; m];
        for i in 0..m {
            let l = queries[i][0] as usize;
            let r = (queries[i][1] + 1) as usize;
            let length = (cnt[r] - cnt[l]) as usize;
            let val_x = (x[r] - x[l] * pow10[length] % MOD + MOD) % MOD;
            let val_sum = (sum[r] - sum[l]) as i64;
            res[i] = ((val_x * val_sum) % MOD) as i32;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            "10203004",
            vec![vec![0, 7], vec![1, 3], vec![4, 6]],
            vec![12340, 4, 9],
        ),
        ("1000", vec![vec![0, 3], vec![1, 1]], vec![1, 0]),
        ("9876543210", vec![vec![0, 9]], vec![444444137]),
    ];

    for (s, queries, expected) in tests {
        assert_eq!(Solution::sum_and_multiply(s.to_string(), queries), expected);
    }
}
