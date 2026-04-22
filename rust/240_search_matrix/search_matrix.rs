/*
 * @Date: 2021-10-25 00:50:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-25 01:35:17
 */

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len() as i32, matrix.first().unwrap().len() as i32);
        let (mut x, mut y) = (0, n - 1);
        while x < m && y >= 0 {
            if matrix[x as usize][y as usize] == target {
                return true;
            } else if matrix[x as usize][y as usize] < target {
                x += 1
            } else if matrix[x as usize][y as usize] > target {
                y -= 1
            }
        }
        false
    }
}

fn main() {
    {
        let matrix: Vec<Vec<i32>> = [
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30],
        ]
        .iter()
        .map(|&x| x.iter().cloned().collect())
        .collect();
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));
    }
    {
        let matrix: Vec<Vec<i32>> = [
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30],
        ]
        .iter()
        .map(|&x| x.iter().cloned().collect())
        .collect();
        let target = 20;
        assert!(!Solution::search_matrix(matrix, target));
    }
    {
        let matrix: Vec<Vec<i32>> = [[-10]]
            .iter()
            .map(|&x| x.iter().cloned().collect())
            .collect();
        let target = -5;
        assert!(!Solution::search_matrix(matrix, target));
    }
}
