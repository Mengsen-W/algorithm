struct Solution;

fn segment_cost(s: &[u8], l: usize, m: usize, factors: &Vec<Vec<usize>>) -> i32 {
    let mut best = m as i32;
    for &d in &factors[m] {
        let groups = m / d; // 每组有 groups 个字符
        let mut changes = 0;
        for offset in 0..d {
            for t in 0..groups / 2 {
                // 组内首尾配对
                if s[l + offset + t * d] != s[l + offset + (groups - 1 - t) * d] {
                    changes += 1;
                }
            }
        }
        best = best.min(changes);
    }
    best
}

fn solve(
    seg: usize,
    start: usize,
    n: usize,
    cost: &Vec<Vec<i32>>,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if seg == 1 {
        // 只剩一段
        return cost[start][n - 1];
    }
    if memo[seg][start] != -1 {
        // 之前算过
        return memo[seg][start];
    }
    let mut best = n as i32;
    let limit = n - 1 - 2 * (seg - 1); // 第一段之后还要留出 seg-1 段
    for e in (start + 1)..=limit {
        best = best.min(cost[start][e] + solve(seg - 1, e + 1, n, cost, memo));
    }
    memo[seg][start] = best;
    best
}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut factors = vec![vec![]; n + 1];
        // 预处理每个数的真因子
        for d in 1..=n / 2 {
            let mut m = d * 2;
            while m <= n {
                factors[m].push(d);
                m += d;
            }
        }
        let mut cost = vec![vec![0i32; n]; n];
        for l in 0..n - 1 {
            for r in (l + 1)..n {
                // 子串长度至少为 2
                cost[l][r] = segment_cost(s, l, r - l + 1, &factors);
            }
        }
        let mut memo = vec![vec![-1i32; n]; k + 1];
        solve(k, 0, n, &cost, &mut memo)
    }
}

fn main() {
    let tests = vec![("abcac", 2, 1), ("abcdef", 2, 2), ("aabbaa", 3, 0)];

    for (s, k, ans) in tests {
        assert_eq!(Solution::minimum_changes(s.to_string(), k), ans);
    }
}
