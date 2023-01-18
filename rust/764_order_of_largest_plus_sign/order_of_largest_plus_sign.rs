/*
 * @Date: 2022-11-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-09
 * @FilePath: /algorithm/764_order_of_largest_plus_sign/order_of_largest_plus_sign.rs
 */

pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for i in 0..n as usize {
        grid.push(vec![1; n as usize]);
    }
    for v in mines {
        grid[v[0] as usize][v[1] as usize] = 0;
    }
    let mut k = 0;

    for i in 0..n as usize {
        for j in 0..n as usize {
            let mut kk: usize = 0;
            while i >= kk
                && j >= kk
                && i + kk < n as usize
                && j + kk < n as usize
                && grid[i - kk][j] > 0
                && grid[i + kk][j] > 0
                && grid[i][j - kk] > 0
                && grid[i][j + kk] > 0
            {
                kk += 1;
            }
            k = std::cmp::max(k, kk as i32);
        }
    }
    k
}

fn main() {
    {
        let n = 5;
        let mines = vec![vec![4, 2]];
        let ans = 2;
        assert_eq!(order_of_largest_plus_sign(n, mines), ans);
    }

    {
        let n = 1;
        let mines = vec![vec![0, 0]];
        let ans = 0;
        assert_eq!(order_of_largest_plus_sign(n, mines), ans);
    }
}
