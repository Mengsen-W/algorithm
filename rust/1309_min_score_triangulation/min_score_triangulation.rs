struct Solution;

impl Solution {
    pub fn min_score_triangulation(v: Vec<i32>) -> i32 {
        fn dfs(i: usize, j: usize, v: &[i32], memo: &mut [Vec<i32>]) -> i32 {
            if i + 1 == j {
                return 0; // 只有两个点，无法组成三角形
            }
            if memo[i][j] != -1 {
                // 之前计算过
                return memo[i][j];
            }
            let mut res = i32::MAX;
            for k in i + 1..j {
                // 枚举顶点 k
                let val = dfs(i, k, v, memo) + dfs(k, j, v, memo) + v[i] * v[j] * v[k];
                res = res.min(val);
            }
            memo[i][j] = res; // 记忆化
            res
        }

        let n = v.len();
        let mut memo = vec![vec![-1; n]; n]; // -1 表示没有计算过
        dfs(0, n - 1, &v, &mut memo)
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 6),
        (vec![3, 7, 4, 5], 144),
        (vec![1, 3, 1, 4, 1, 5], 13),
    ];

    for (v, ans) in tests {
        assert_eq!(Solution::min_score_triangulation(v), ans);
    }
}
