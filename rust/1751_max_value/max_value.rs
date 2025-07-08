struct Solution;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_by_key(|e| e[1]);
        let n = events.len();
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        let _ends: Vec<i32> = events.iter().map(|e| e[1]).collect();

        for i in 0..n {
            let p = events.partition_point(|x| x[1] < events[i][0]);
            for j in 1..=k {
                dp[i + 1][j] = dp[i][j].max(dp[p][j - 1] + events[i][2]);
            }
        }

        dp[n][k]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2, 7),
        (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2, 10),
        (
            vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
            3,
            9,
        ),
    ];

    for (events, k, expected) in tests {
        assert_eq!(Solution::max_value(events, k), expected);
    }
}
