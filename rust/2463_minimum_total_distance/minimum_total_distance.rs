struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();
        let n = robot.len();
        let m = factory.len();
        let mut dp = vec![vec![i64::MAX / 2; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = 0;
        }
        for j in 1..=m {
            for i in 1..=n {
                dp[i][j] = dp[i][j - 1];
                let mut cost: i64 = 0;
                let lim = i.min(factory[j - 1][1] as usize);
                for k in 1..=lim {
                    cost += (robot[i - k] as i64 - factory[j - 1][0] as i64).abs();
                    dp[i][j] = dp[i][j].min(dp[i - k][j - 1] + cost);
                }
            }
        }
        dp[n][m]
    }
}

fn main() {
    let tests = vec![
        (vec![0,4,6], vec![vec![2,2], vec![6,2]],4),
        (vec![1, -1], vec![vec![-2,1], vec![2,1]],2),
    ];

    for (robot, factory, ans) in tests {
        assert_eq!(Solution::minimum_total_distance(robot, factory), ans);
    }
}
