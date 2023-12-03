/*
 * @Date: 2023-12-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-03
 * @FilePath: /algorithm/rust/1423_max_score/max_score.rs
 */

struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut s = card_points.iter().take(k).sum::<i32>();
        let mut ans = s;
        for i in 1..=k {
            s += card_points[card_points.len() - i] - card_points[k - i];
            ans = ans.max(s);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5, 6, 1], 3, 12),
        (vec![2, 2, 2], 2, 4),
        (vec![9, 7, 7, 9, 7, 7, 9], 7, 55),
        (vec![1, 1000, 1], 1, 1),
        (vec![1, 79, 80, 1, 1, 1, 200, 1], 3, 202),
    ];

    for (card_points, k, ans) in tests {
        assert_eq!(Solution::max_score(card_points, k), ans);
    }
}
