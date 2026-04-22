/*
 * @Date: 2024-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-22
 * @FilePath: /algorithm/rust/2225_find_winners/find_winners.rs
 */

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        for match_ in matches {
            let (winner, loser) = (match_[0], match_[1]);
            freq.entry(winner).or_insert(0);
            *freq.entry(loser).or_insert(0) += 1;
        }

        let mut ans = vec![Vec::new(), Vec::new()];
        for (key, value) in freq {
            if value < 2 {
                ans[value as usize].push(key);
            }
        }
        ans[0].sort_unstable();
        ans[1].sort_unstable();
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9],
            ],
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]],
        ),
        (
            vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]],
            vec![vec![1, 2, 5, 6], vec![]],
        ),
    ];

    for (matches, ans) in tests {
        assert_eq!(Solution::find_winners(matches), ans);
    }
}
