struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![i32::MIN; n + 1];

        dp[n] = 0;

        for i in (0..n).rev() {
            let mut current_sum = 0;
            for j in i + 1..=(i + 3).min(n) {
                current_sum += stone_value[j - 1];
                dp[i] = dp[i].max(current_sum - dp[j])
            }
        }

        if dp[0] == 0 {
            "Tie".to_string()
        } else if dp[0] > 0 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 7], "Bob"),
        (vec![1, 2, 3, -9], "Alice"),
        (vec![1, 2, 3, 6], "Tie"),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::stone_game_iii(test), expected);
    }
}
