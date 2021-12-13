/*
 * @Date: 2021-12-13 01:39:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-13 01:55:06
 */

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut vert = vec![0; n];
    let mut horz = vec![0; n];
    for i in 0..n {
        let mut max_v = -1;
        let mut max_h = -1;
        for j in 0..n {
            max_v = max_v.max(grid[j][i]);
            max_h = max_h.max(grid[i][j]);
        }
        vert[i] = max_v;
        horz[i] = max_h;
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            let min = i32::min(horz[i], vert[j]);
            if grid[i][j] < min {
                ans += min - grid[i][j];
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0]
        ]),
        35
    );
}
