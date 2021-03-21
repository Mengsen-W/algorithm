/*
 * @Date: 2021-03-21 09:36:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-21 10:05:00
 */

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut zero_row, mut zero_col) = (vec![false; m], vec![false; n]);
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                zero_row[i] = true;
                zero_col[j] = true;
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if zero_row[i] || zero_col[j] {
                matrix[i][j] = 0;
            }
        }
    }
}

fn main() {
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes(&mut matrix);
    assert_eq!(
        matrix,
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
    )
}
