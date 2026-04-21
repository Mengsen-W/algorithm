struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n + 1]; // 解决每道题及以后题目的最高分数
        for i in (0..n).rev() {
            dp[i] = dp[i + 1]
                .max(questions[i][0] as i64 + dp[(n).min(i + questions[i][1] as usize + 1)]);
        }
        dp[0]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]], 5),
        (
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
            7,
        ),
    ];

    for (questions, ans) in tests {
        assert_eq!(Solution::most_points(questions), ans);
    }
}
