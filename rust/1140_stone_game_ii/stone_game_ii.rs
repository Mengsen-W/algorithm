struct Solution;

impl Solution {
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
}

fn main() {
    let tests = vec![(vec![2, 7, 9, 4, 4], 10), (vec![1, 2, 3, 4, 5, 100], 104)];

    for (piles, expected) in tests {
        assert_eq!(Solution::stone_game_ii(piles), expected);
    }
}
