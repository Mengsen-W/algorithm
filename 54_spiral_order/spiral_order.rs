/*
 * @Date: 2021-03-15 08:41:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-15 08:50:45
 * @FilePath: \algorithm\54_spiral_order\spiral_order.rs
 */

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    if matrix.is_empty() {
        return res;
    }
    if matrix[0].is_empty() {
        return res;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut left = 0;
    let mut right = cols;
    let mut top = 0;
    let mut down = rows;
    while res.len() < cols * rows {
        for j in left..right {
            res.push(matrix[top][j]);
        }
        top += 1;
        for i in top..down {
            res.push(matrix[i][right - 1]);
        }
        right -= 1;
        if down <= top {
            break;
        }
        for j in (left..right).rev() {
            res.push(matrix[down - 1][j]);
        }
        down -= 1;
        if left >= right {
            break;
        }
        for i in (top..down).rev() {
            res.push(matrix[i][left]);
        }
        left += 1;
    }
    res
}

fn main() {
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
