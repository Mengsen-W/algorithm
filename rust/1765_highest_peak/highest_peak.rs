/*
 * @Date: 2022-01-29 00:52:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-29 01:09:05
 */

struct Solution;

impl Solution {
    pub fn highest_peak(mut height: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (height.len(), height[0].len());
        use std::collections::VecDeque;
        let mut dq = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if height[i][j] == 1 {
                    height[i][j] = 0;
                    dq.push_back((i, j));
                } else {
                    height[i][j] = -1;
                }
            }
        }

        while let Some((i, j)) = dq.pop_front() {
            let h = height[i][j] + 1;
            if i > 0 && height[i - 1][j] == -1 {
                height[i - 1][j] = h;
                dq.push_back((i - 1, j))
            }
            if i < m - 1 && height[i + 1][j] == -1 {
                height[i + 1][j] = h;
                dq.push_back((i + 1, j))
            }
            if j > 0 && height[i][j - 1] == -1 {
                height[i][j - 1] = h;
                dq.push_back((i, j - 1))
            }
            if j < n - 1 && height[i][j + 1] == -1 {
                height[i][j + 1] = h;
                dq.push_back((i, j + 1))
            }
        }

        height
    }
}

fn main() {
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]),
        vec![vec![1, 0], vec![2, 1]]
    );
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
        vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
    );
}
