struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let length = piles.len();
        let mut dp: Vec<i32> = vec![0; length];
        for i in (0..(length - 1)).rev() {
            dp[i] = piles[i];
            for j in i + 1..length {
                dp[j] = (piles[i] - dp[j]).max(piles[j] - dp[j - 1]);
            }
        }
        dp[length - 1] > 0
    }
}

fn main() {
    let tests = vec![(vec![5, 3, 4, 5], true), (vec![3, 7, 2, 3], true)];

    for (piles, expected) in tests {
        assert_eq!(Solution::stone_game(piles), expected);
    }
}
