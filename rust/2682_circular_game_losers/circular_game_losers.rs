/*
 * @Date: 2023-08-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-16
 * @FilePath: /algorithm/rust/2682_circular_game_losers/circular_game_losers.rs
 */

struct Solution;
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut vis: Vec<bool> = vec![false; n as usize];

        let mut i = 0;
        let mut p = 1;
        while !vis[i] {
            vis[i] = true;
            i = (i + p * k as usize) % n as usize;
            p += 1;
        }

        let mut ans = Vec::new();
        for i in 0..vis.len() {
            if !vis[i] {
                ans.push((i + 1) as i32);
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![(5, 2, vec![4, 5]), (4, 4, vec![2, 3, 4])];

    for (n, k, ans) in tests {
        assert_eq!(Solution::circular_game_losers(n, k), ans);
    }
}
