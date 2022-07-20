/*
 * @Date: 2022-07-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-20
 * @FilePath: /algorithm/1260_shift_grid/shift_grid.rs
 */

pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut a: Vec<i32> = grid.into_iter().flatten().collect();
    a.rotate_right(k as usize % (m * n));
    a.chunks(n).map(|v| v.to_vec()).collect()
}

fn main() {
    {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let ans = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        assert_eq!(shift_grid(grid, k), ans);
    }

    {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 1],
        ];
        let k = 4;
        let ans = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        assert_eq!(shift_grid(grid, k), ans);
    }

    {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 9;
        let ans = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(shift_grid(grid, k), ans)
    }
}
