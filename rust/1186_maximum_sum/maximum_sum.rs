/*
 * @Date: 2023-06-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-27
 * @FilePath: /algorithm/rust/1186_maximum_sum/maximum_sum.rs
 */

pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    const MIN: i32 = -10000;
    let n = arr.len();
    let mut dp = vec![vec![MIN; 2]; n + 1];
    let mut ans = MIN;

    for i in 0..n {
        dp[i + 1][0] = (dp[i][0] + arr[i]).max(arr[i]);
        dp[i + 1][1] = (dp[i][1] + arr[i]).max(dp[i][0]);
        ans = ans.max(dp[i + 1][0]).max(dp[i + 1][1]);
    }

    ans
}

fn main() {
    let test_map = vec![
        (vec![1, -2, 0, 3], 4),
        (vec![1, -2, -2, 3], 3),
        (vec![-1, -1, -1, -1], -1),
    ];
    for (arr, ans) in test_map {
        assert_eq!(maximum_sum(arr), ans);
    }
}
