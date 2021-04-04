/*
 * @Date: 2021-04-04 18:14:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:36:38
 * @FilePath: \algorithm\74_search_matrix\search_matrix.rs
 * @Description: file content
 */

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix.into_iter().flatten().any(|x| x == target)
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(search_matrix(matrix, 3));
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(!search_matrix(matrix, 13));
}
