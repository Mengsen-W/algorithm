/*
 * @Date: 2021-12-19 01:00:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-19 01:29:46
 */

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut in_degree = vec![0; n as usize + 1];
        let mut out_degree = vec![0; n as usize + 1];

        for edge in trust {
            in_degree[edge[1] as usize] += 1;
            out_degree[edge[0] as usize] += 1;
        }

        (1..=n)
            .filter(|&i| in_degree[i as usize] == n - 1 && out_degree[i as usize] == 0)
            .nth(0)
            .unwrap_or(-1) as i32
    }
}

fn main() {
    let tests = vec![
        (2, vec![vec![1, 2]], 2),
        (3, vec![vec![1, 3], vec![2, 3]], 3),
        (3, vec![vec![1, 3], vec![2, 3], vec![3, 1]], -1),
        (3, vec![vec![1, 2], vec![2, 3]], -1),
        (
            4,
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]],
            3,
        ),
    ];

    for (n, trust, ans) in tests {
        assert_eq!(Solution::find_judge(n, trust), ans);
    }
}
