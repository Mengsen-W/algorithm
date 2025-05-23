struct Solution;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const MAX_N: usize = 10_000 + 10;
        const MAX_P: usize = 15; // 最多有 15 个质因子

        let mut sieve = vec![0; MAX_N]; // 最小质因子
        for i in 2..MAX_N {
            if sieve[i] == 0 {
                for j in (i..MAX_N).step_by(i) {
                    sieve[j] = i as i32;
                }
            }
        }

        let mut ps = vec![vec![]; MAX_N]; // 质因子个数列表
        for i in 2..=max_value as usize {
            let mut x = i;
            while x > 1 {
                let p = sieve[x] as usize;
                let mut cnt = 0;
                while x % p == 0 {
                    x /= p;
                    cnt += 1;
                }
                ps[i].push(cnt);
            }
        }

        let mut c = vec![vec![0; MAX_P + 1]; n as usize + MAX_P + 1];
        c[0][0] = 1;
        for i in 1..n as usize + MAX_P + 1 {
            c[i][0] = 1;
            for j in 1..=i.min(MAX_P) {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % MOD;
            }
        }

        let mut ans = 0i64;
        let n = n as usize;
        for x in 1..=max_value as usize {
            let mut mul = 1i64;
            for &p in &ps[x] {
                mul = mul * c[n + p as usize - 1][p as usize] % MOD;
            }
            ans = (ans + mul) % MOD;
        }

        ans as i32
    }
}

fn main() {
    let tests = vec![(2, 5, 10), (5, 3, 11)];

    for (n, max_value, expected) in tests {
        assert_eq!(
            Solution::ideal_arrays(n, max_value),
            expected,
            "n: {}, max_value: {}, expected: {}",
            n,
            max_value,
            expected
        );
    }
}
