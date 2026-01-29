struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const INF: i32 = i32::MAX / 2;
        let mut g = [[INF; 26]; 26];
        for i in 0..26 {
            g[i][i] = 0;
        }

        for i in 0..original.len() {
            let idx = (original[i] as u8 - b'a') as usize;
            let idy = (changed[i] as u8 - b'a') as usize;
            g[idx][idy] = g[idx][idy].min(cost[i]);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if g[i][k] != INF && g[k][j] != INF {
                        g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                    }
                }
            }
        }

        let mut ans: i64 = 0;
        let source_bytes = source.as_bytes();
        let target_bytes = target.as_bytes();

        for i in 0..source_bytes.len() {
            let idx = (source_bytes[i] - b'a') as usize;
            let idy = (target_bytes[i] - b'a') as usize;
            if g[idx][idy] == INF {
                return -1;
            }
            ans += g[idx][idy] as i64;
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            "abcd",
            "acbe",
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20],
            28,
        ),
        (
            "aaaa",
            "bbbb",
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2],
            12,
        ),
        ("abcd", "acbe", vec!['a'], vec!['e'], vec![10000], -1),
    ];

    for (source, target, original, changed, cost, ans) in tests {
        assert_eq!(
            Solution::minimum_cost(
                source.to_string(),
                target.to_string(),
                original,
                changed,
                cost
            ),
            ans
        );
    }
}
