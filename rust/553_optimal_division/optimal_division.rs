/*
 * @Date: 2022-02-27 00:38:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-27 02:55:05
 * @FilePath: /algorithm/553_optimal_division/optimal_division.rs
 * @Description: file content
 */

pub fn optimal_division1(nums: Vec<i32>) -> String {
    #[derive(Clone)]
    struct Node {
        max_val: f32,
        min_val: f32,
        max_str: String,
        min_str: String,
    }
    impl Node {
        fn new() -> Self {
            Node {
                max_val: 0.0,
                min_val: 10000.0,
                max_str: String::new(),
                min_str: String::new(),
            }
        }
    }

    let n = nums.len();
    let mut dp = vec![vec![Node::new(); n]; n];
    nums.iter().enumerate().for_each(|(i, &num)| {
        dp[i][i].max_val = num as f32;
        dp[i][i].min_val = num as f32;
        dp[i][i].max_str = num.to_string();
        dp[i][i].min_str = num.to_string();
    });
    for i in 1..n {
        for j in 0..n - i {
            for k in j..j + i {
                if dp[j][j + i].max_val < dp[j][k].max_val / dp[k + 1][j + i].min_val {
                    dp[j][j + i].max_val = dp[j][k].max_val / dp[k + 1][j + i].min_val;
                    if k + 1 == j + i {
                        dp[j][j + i].max_str =
                            format!("{}/{}", dp[j][k].max_str, dp[k + 1][j + i].min_str);
                    } else {
                        dp[j][j + i].max_str =
                            format!("{}/({})", dp[j][k].max_str, dp[k + 1][j + i].min_str);
                    }
                }
                if dp[j][j + i].min_val > dp[j][k].min_val / dp[k + 1][j + i].max_val {
                    dp[j][j + i].min_val = dp[j][k].min_val / dp[k + 1][j + i].max_val;
                    if k + 1 == j + i {
                        dp[j][j + i].min_str =
                            format!("{}/{}", dp[j][k].min_str, dp[k + 1][j + i].max_str);
                    } else {
                        dp[j][j + i].min_str =
                            format!("{}/({})", dp[j][k].min_str, dp[k + 1][j + i].max_str);
                    }
                }
            }
        }
    }
    dp[0][n - 1].max_str.clone()
}

pub fn optimal_division2(nums: Vec<i32>) -> String {
    match nums.len() {
        1 => nums[0].to_string(),
        2 => format!("{}/{}", nums[0], nums[1]),
        _ => format!(
            "{}/({})",
            nums[0],
            nums[1..]
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("/")
        ),
    }
}

fn main() {
    assert_eq!(
        optimal_division1(vec![1000, 100, 10, 2]),
        String::from("1000/(100/10/2)")
    );
    assert_eq!(
        optimal_division2(vec![1000, 100, 10, 2]),
        String::from("1000/(100/10/2)")
    );
}
