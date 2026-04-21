/*
 * @Date: 2023-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-14
 * @FilePath: /algorithm/rust/1605_restore_matrix/restore_matrix.rs
 */

pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut i, mut j, n, m) = (0, 0, row_sum.len(), col_sum.len());
    let mut minimum;
    let mut ans = vec![vec![0; m]; n];
    while i < n && j < m {
        minimum = row_sum[i].min(col_sum[j]);
        ans[i][j] = minimum;
        row_sum[i] -= minimum;
        col_sum[j] -= minimum;
        if row_sum[i] == 0 {
            i += 1;
        }
        if col_sum[j] == 0 {
            j += 1;
        }
    }
    ans
}

fn main() {
    {
        let row_sum = vec![3, 8];
        let col_sum = vec![4, 7];
        let ans = vec![vec![3, 0], vec![1, 7]];
        assert_eq!(restore_matrix(row_sum, col_sum), ans);
    }

    {
        let row_sum = vec![5, 7, 10];
        let col_sum = vec![8, 6, 8];
        let ans: Vec<Vec<i32>> = [[5, 0, 0], [3, 4, 0], [0, 2, 8]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(restore_matrix(row_sum, col_sum), ans);
    }

    {
        let row_sum = vec![14, 9];
        let col_sum = vec![6, 9, 8];
        let ans: Vec<Vec<i32>> = [[6, 8, 0], [0, 1, 8]].iter().map(|v| v.to_vec()).collect();
        assert_eq!(restore_matrix(row_sum, col_sum), ans);
    }

    {
        let row_sum = vec![1, 0];
        let col_sum = vec![1];
        let ans = vec![vec![1], vec![0]];
        assert_eq!(restore_matrix(row_sum, col_sum), ans);
    }

    {
        let row_sum = vec![0];
        let col_sum = vec![0];
        let ans = vec![vec![0]];
        assert_eq!(restore_matrix(row_sum, col_sum), ans);
    }
}
