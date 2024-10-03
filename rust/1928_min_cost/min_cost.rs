struct Solution;

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        use std::cmp::min;
        let n = passing_fees.len();
        let mut f = vec![vec![i32::MAX; n]; (max_time + 1) as usize];
        f[0][0] = passing_fees[0];

        for t in 1..=max_time {
            for edge in &edges {
                let (i, j, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
                if cost <= t {
                    if f[(t - cost) as usize][j] != i32::MAX {
                        f[t as usize][i] = min(
                            f[t as usize][i],
                            f[(t - cost) as usize][j] + passing_fees[i],
                        );
                    }
                    if f[(t - cost) as usize][i] != i32::MAX {
                        f[t as usize][j] = min(
                            f[t as usize][j],
                            f[(t - cost) as usize][i] + passing_fees[j],
                        );
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for t in 1..=max_time {
            ans = min(ans, f[t as usize][n - 1]);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    let tests = vec![
        (
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            11,
        ),
        (
            29,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            48,
        ),
        (
            25,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
            -1,
        ),
    ];

    for (max_time, edges, passing_fees, ans) in tests {
        assert_eq!(Solution::min_cost(max_time, edges, passing_fees), ans);
    }
}
