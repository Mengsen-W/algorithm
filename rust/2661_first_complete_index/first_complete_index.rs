/*
 * @Date: 2023-12-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-02
 * @FilePath: /algorithm/rust/2661_first_complete_index/first_complete_index.rs
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut idx = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                idx.insert(mat[i][j], [i, j]);
            }
        }

        let mut row = vec![0; m];
        let mut col = vec![0; n];
        for k in 0..arr.len() {
            let x = idx.get(&arr[k]).unwrap();
            let i = x[0];
            let j = x[1];
            row[i] += 1;
            col[j] += 1;
            if row[i] == n || col[j] == m {
                return k as i32;
            }
        }

        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]], 2),
        (
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]],
            3,
        ),
    ];

    for (arr, mat, ans) in tests {
        assert_eq!(Solution::first_complete_index(arr, mat), ans);
    }
}
