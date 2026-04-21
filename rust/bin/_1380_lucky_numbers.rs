/*
 * @Date: 2022-02-15 02:05:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-15 02:41:52
 * @FilePath: /algorithm/1380_lucky_numbers/lucky_numbers.rs
 */

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut min_row = vec![i32::MAX; matrix.len()];
    let mut max_col = vec![i32::MIN; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            min_row[i] = min_row[i].min(matrix[i][j]);
            max_col[j] = max_col[j].max(matrix[i][j]);
        }
    }
    let mut ans = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == min_row[i] && matrix[i][j] == max_col[j] {
                ans.push(matrix[i][j]);
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );

    assert_eq!(
        lucky_numbers(vec![
            vec![1, 10, 4, 2],
            vec![9, 3, 8, 7],
            vec![15, 16, 17, 12]
        ]),
        vec![12]
    );

    assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
}
