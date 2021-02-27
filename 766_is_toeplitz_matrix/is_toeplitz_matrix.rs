/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-27 17:55:14
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-27 20:28:37
 */

fn is_toeplitz_matrix(matrix: &Vec<Vec<i32>>) -> bool {
    matrix.windows(2).all(|x| {
        x[1][1..]
            .iter()
            .copied()
            .zip(x[0].iter().copied())
            .all(|(x, y)| x == y)
    })
}

fn main() {
    let mut matrix;
    matrix = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
    assert!(is_toeplitz_matrix(&matrix));
    matrix = vec![vec![1, 2], vec![2, 2]];
    assert!(!is_toeplitz_matrix(&matrix));
}
