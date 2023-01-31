/*
 * @Date: 2023-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-31
 * @FilePath: /algorithm/rust/2319_check_x_matrix/check_x_matrix.rs
 */

pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    grid.into_iter().enumerate().all(|(n, x)| {
        x.iter()
            .enumerate()
            .all(|(m, &y)| (y == 0) ^ (m == n || m + n == x.len() - 1))
    })
}

fn main() {
    {
        let grid = [[2, 0, 0, 1], [0, 3, 1, 0], [0, 5, 2, 0], [4, 0, 0, 2]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let ans = true;
        assert_eq!(check_x_matrix(grid), ans);
    }

    {
        let grid = [[5, 7, 0], [0, 3, 1], [0, 5, 0]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let ans = false;
        assert_eq!(check_x_matrix(grid), ans);
    }
}
