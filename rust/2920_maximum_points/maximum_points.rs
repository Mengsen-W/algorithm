struct Solution;

impl Solution {
    fn dfs(
        node: usize,
        parent: Option<usize>,
        f: usize,
        coins: &Vec<i32>,
        k: i32,
        children: &Vec<Vec<usize>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if memo[node][f] >= 0 {
            return memo[node][f];
        }
        let mut res0 = (coins[node] >> f) - k;
        let mut res1 = coins[node] >> (f + 1);
        for &child in &children[node] {
            if Some(child) == parent {
                continue;
            }
            res0 += Solution::dfs(child, Some(node), f, coins, k, children, memo);
            if f + 1 < 14 {
                res1 += Solution::dfs(child, Some(node), f + 1, coins, k, children, memo);
            }
        }
        memo[node][f] = res0.max(res1);
        memo[node][f]
    }

    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut children = vec![vec![]; n];
        for edge in edges {
            children[edge[0] as usize].push(edge[1] as usize);
            children[edge[1] as usize].push(edge[0] as usize);
        }
        let mut memo = vec![vec![-1; 14]; n];
        Solution::dfs(0, None, 0, &coins, k, &children, &mut memo)
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            vec![10, 10, 3, 3],
            5,
            11,
        ),
        (vec![vec![0, 1], vec![0, 2]], vec![8, 4, 4], 0, 16),
    ];

    for (edges, coins, k, ans) in tests {
        assert_eq!(Solution::maximum_points(edges, coins, k), ans);
    }
}
