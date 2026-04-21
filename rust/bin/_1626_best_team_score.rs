/*
 * @Date: 2023-03-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-22
 * @FilePath: /algorithm/rust/1626_best_team_score/best_team_score.rs
 */

pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut index = (0..scores.len()).collect::<Vec<_>>();
    index.sort_unstable_by(|&a, &b| {
        if ages[a] == ages[b] {
            scores[b].cmp(&scores[a])
        } else {
            ages[b].cmp(&ages[a])
        }
    });
    let (mut dp, mut maximum) = (vec![0; scores.len()], 0);
    for i in 0..scores.len() {
        dp[i] = scores[index[i]];
        for j in 0..i {
            if scores[index[i]] <= scores[index[j]] {
                dp[i] = dp[i].max(dp[j] + scores[index[i]]);
            }
        }
        maximum = maximum.max(dp[i]);
    }
    maximum
}

fn main() {
    {
        let scores = vec![1, 3, 5, 10, 15];
        let ages = vec![1, 2, 3, 4, 5];
        let ans = 34;
        assert_eq!(best_team_score(scores, ages), ans);
    }

    {
        let scores = vec![4, 5, 6, 5];
        let ages = vec![2, 1, 2, 1];
        let ans = 16;
        assert_eq!(best_team_score(scores, ages), ans);
    }

    {
        let scores = vec![1, 2, 3, 5];
        let ages = vec![8, 9, 10, 1];
        let ans = 6;
        assert_eq!(best_team_score(scores, ages), ans);
    }
}
