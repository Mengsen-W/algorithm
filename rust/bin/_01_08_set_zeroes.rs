/*
 * @Date: 2022-09-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-30
 * @FilePath: /algorithm/01.08_set_zeroes/set_zeroes.rs
 */

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    if matrix.is_empty() {
        return;
    }
    let mut rows = vec![-1; matrix.len()];
    let mut cols = vec![-1; matrix[0].len()];
    for r in 0..rows.len() {
        for c in 0..cols.len() {
            if matrix[r][c] == 0 {
                rows[r] = 0;
                cols[c] = 0;
            }
        }
    }

    for r in 0..rows.len() {
        for c in 0..cols.len() {
            matrix[r][c] &= rows[r] & cols[c];
        }
    }
}

fn main() {
    {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let ans = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, ans);
    }

    {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let ans = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, ans);
    }
}
