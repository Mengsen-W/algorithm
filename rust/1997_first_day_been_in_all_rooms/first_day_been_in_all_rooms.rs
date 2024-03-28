/*
 * @Date: 2024-03-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-28
 * @FilePath: /algorithm/rust/1997_first_day_been_in_all_rooms/first_day_been_in_all_rooms.rs
 */

struct Solution;

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp: Vec<i32> = vec![0; next_visit.len()];

        // 初始化原地待一天+访问下一个房间一天
        dp[0] = 2;
        for i in 1..next_visit.len() {
            let to = next_visit[i] as usize;
            dp[i] = 2 + dp[i - 1];
            if to != 0 {
                // 避免负数
                dp[i] = (dp[i] - dp[to - 1] + MOD) % MOD;
            }
            dp[i] = (dp[i] + dp[i - 1]) % MOD;
        }
        return dp[next_visit.len() - 2]; //题目保证n >= 2
    }
}

fn main() {
    let tests = vec![(vec![0, 0], 2), (vec![0, 0, 2], 6), (vec![0, 1, 2, 0], 6)];

    for (next_visit, ans) in tests {
        assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), ans);
    }
}
