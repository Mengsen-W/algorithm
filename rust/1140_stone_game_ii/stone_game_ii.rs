/*
 * @Date: 2023-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-22
 * @FilePath: /algorithm/rust/1140_stone_game_ii/stone_game_ii.rs
 */

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let len = piles.len();
    let mut sum = 0;
    // dp[i][j]表示当前是第i波，m = j;
    let mut dp = vec![vec![0; len + 1]; len + 1];
    for i in (0..len).rev() {
        sum += piles[i]; // 表示当前所剩下的所有棋子的和
        for m in (1..len + 1).rev() {
            if i + 2 * m >= len {
                dp[i][m] = sum;
                continue;
            }
            for x in 1..((len - i).min(2 * m) + 1) {
                dp[i][m] = dp[i][m].max(sum - dp[i + x][m.max(x)]);
            }
        }
    }
    dp[0][1]
}

fn main() {
    {
        let piles = vec![2, 7, 9, 4, 4];
        let ans = 10;
        assert_eq!(stone_game_ii(piles), ans);
    }

    {
        let piles = vec![1, 2, 3, 4, 5, 100];
        let ans = 104;
        assert_eq!(stone_game_ii(piles), ans);
    }
}
