struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let n = prize_positions.len();
        let mut dp = vec![0; n + 1];
        let mut ans = 0;
        let mut left = 0;
        for right in 0..n {
            while prize_positions[right] - prize_positions[left] > k {
                left += 1;
            }
            ans = ans.max(right as i32 - left as i32 + 1 + dp[left]);
            dp[right + 1] = dp[right].max(right as i32 - left as i32 + 1);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 2, 2, 3, 3, 5], 2, 7), (vec![1, 2, 3, 4], 0, 2)];

    for (prize_positions, k, ans) in tests {
        assert_eq!(Solution::maximize_win(prize_positions, k), ans);
    }
}
