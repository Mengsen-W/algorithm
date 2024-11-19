struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prev: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        let mut dp: Vec<i32> = vec![0; n as usize];
        for i in 1..n {
            prev[i as usize].push(i - 1);
            dp[i as usize] = i;
        }
        let mut res: Vec<i32> = Vec::new();
        for query in queries {
            prev[query[1] as usize].push(query[0]);
            for v in query[1] as usize..n as usize {
                for &u in &prev[v] {
                    dp[v] = dp[v].min(dp[u as usize] + 1);
                }
            }
            res.push(dp[(n - 1) as usize]);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (5, vec![vec![2, 4], vec![0, 2], vec![0, 4]], vec![3, 2, 1]),
        (4, vec![vec![0, 3], vec![0, 2]], vec![1, 1]),
    ];

    for (n, queries, ans) in tests {
        assert_eq!(Solution::shortest_distance_after_queries(n, queries), ans);
    }
}
