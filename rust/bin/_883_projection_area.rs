/*
 * @Date: 2022-04-26 09:35:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-26 09:45:40
 * @FilePath: /algorithm/883_projection_area/projection_area.rs
 */

pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let (mut xy_area, mut zx_area, mut yz_area) = (0, 0, 0);
    for i in 0..n {
        let (mut yz_height, mut zx_height) = (0, 0);
        for j in 0..n {
            xy_area += if grid[i][j] > 0 { 1 } else { 0 };
            yz_height = yz_height.max(grid[i][j]);
            zx_height = zx_height.max(grid[j][i]);
        }
        yz_area += yz_height;
        zx_area += zx_height;
    }
    xy_area + zx_area + yz_area
}

fn main() {
    assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(projection_area(vec![vec![2]]), 5);
    assert_eq!(projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}
