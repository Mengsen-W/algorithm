/*
 * @Date: 2023-06-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-20
 * @FilePath: /algorithm/rust/1595_connect_two_groups/connect_two_groups.rs
 */

pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    let n = cost.len();
    let m = cost[0].len();
    let mut dp = vec![vec![1_000_000_007; 1 << m]; n + 1];
    dp[n][(1 << m) - 1] = 0;

    for i in (0..n).rev() {
        for k in (0..(1 << m)).rev() {
            for j in 0..m {
                dp[i][k] = dp[i][k].min(cost[i][j] + dp[i + 1][k | (1 << j)]);

                if k & (1 << j) == 0 {
                    dp[i][k] = dp[i][k].min(cost[i][j] + dp[i][k | (1 << j)]);
                }
            }
        }
    }

    dp[0][0] as i32
}

fn main() {
    {
        let cost = vec![vec![15, 96], vec![36, 2]];
        let ans = 17;
        assert_eq!(connect_two_groups(cost), ans);
    }

    {
        let cost = vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]];
        let ans = 4;
        assert_eq!(connect_two_groups(cost), ans);
    }

    {
        let cost = vec![
            vec![2, 5, 1],
            vec![3, 4, 7],
            vec![8, 1, 2],
            vec![6, 2, 4],
            vec![3, 8, 8],
        ];
        let ans = 10;
        assert_eq!(connect_two_groups(cost), ans);
    }
}
