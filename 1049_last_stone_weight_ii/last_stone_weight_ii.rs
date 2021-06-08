/*
 * @Date: 2021-06-08 08:38:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-08 08:59:20
 */

fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();
    let m = sum as usize / 2;
    let mut dp: Vec<bool> = vec![false; m + 1];
    dp[0] = true;
    for weight in stones {
        for j in ((weight as usize)..=m).rev() {
            dp[j] = dp[j] || dp[j - weight as usize];
        }
    }

    for j in (0..=m).rev() {
        if dp[j] {
            return sum - 2 * j as i32;
        }
    }
    0
}

fn main() {
    {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight_ii(stones), 1);
    }
    {
        let stones = vec![31, 26, 33, 21, 40];
        assert_eq!(last_stone_weight_ii(stones), 5);
    }
    {
        let stones = vec![1, 2];
        assert_eq!(last_stone_weight_ii(stones), 1);
    }
}
