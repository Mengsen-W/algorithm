/*
 * @Date: 2021-06-16 09:09:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-16 09:22:38
 */

fn stone_game(piles: Vec<i32>) -> bool {
    let length = piles.len();
    let mut dp: Vec<i32> = vec![0; length];
    for i in 0..length {
        dp[i] = piles[i];
    }
    for i in (0..(length - 2)).rev() {
        for j in i + 1..length {
            dp[j] = (piles[i] - dp[j]).max(piles[j] - dp[j - 1]);
        }
    }
    dp[length - 1] > 0
}

fn main() {
    assert!(stone_game(vec![3, 7, 2, 3]));
    assert!(stone_game(vec![5, 3, 4, 5]));
}
