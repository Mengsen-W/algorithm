/*
 * @Date: 2021-04-29 09:30:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-29 09:41:05
 * @FilePath: \algorithm\403_can_cross\can_cross.rs
 * @Description: file content
 */

fn can_cross(stones: Vec<i32>) -> bool {
    let n = stones.len();
    let mut dp = vec![vec![false; n]; n];
    dp[0][0] = true;
    for i in 1..n {
        if stones[i] - stones[i - 1] > i as i32 {
            return false;
        }
    }
    for i in 1..n {
        for j in (0..=i - 1).rev() {
            let k = (stones[i] - stones[j]) as usize;
            if k > j + 1 {
                break;
            }
            dp[i][k] = dp[j][k - 1] || dp[j][k] || dp[j][k + 1];
            if i == n - 1 && dp[i][k] {
                return true;
            }
        }
    }
    false
}

fn main() {
    {
        let stones = vec![0, 1, 3, 5, 6, 8, 12, 17];
        assert!(can_cross(stones));
    }
    {
        let stones = vec![0, 1, 2, 3, 4, 8, 9, 11];
        assert!(!can_cross(stones));
    }
}
