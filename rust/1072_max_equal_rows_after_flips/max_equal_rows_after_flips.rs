/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/rust/1072_max_equal_rows_after_flips/max_equal_rows_after_flips.rs
 */

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n, mut cnt) = (matrix.len(), matrix[0].len(), 0);
    for i in 0..m {
        let flip = (0..n).map(|j| 1 - matrix[i][j]).collect::<Vec<_>>();
        cnt = cnt.max(
            (i..m)
                .filter(|&k| matrix[i] == matrix[k] || matrix[k] == flip)
                .count() as i32,
        );
    }
    cnt
}

fn main() {
    {
        let matrix = vec![vec![0, 1], vec![1, 1]];
        let ans = 1;
        assert_eq!(max_equal_rows_after_flips(matrix), ans);
    }

    {
        let matrix = vec![vec![0, 1], vec![1, 0]];
        let ans = 2;
        assert_eq!(max_equal_rows_after_flips(matrix), ans);
    }

    {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
        let ans = 2;
        assert_eq!(max_equal_rows_after_flips(matrix), ans);
    }
}
