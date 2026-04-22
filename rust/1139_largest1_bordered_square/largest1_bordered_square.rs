/*
 * @Date: 2023-02-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-17
 * @FilePath: /algorithm/rust/1139_largest1_bordered_square/largest1_bordered_square.rs
 */

pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut row = grid.clone();
    let mut col = grid;
    for i in 0..n {
        for j in (0..m - 1).rev() {
            if row[i][j] > 0 {
                row[i][j] += row[i][j + 1];
            }
        }
    }
    for i in (0..n - 1).rev() {
        for j in 0..m {
            if col[i][j] > 0 {
                col[i][j] += col[i + 1][j];
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let start = ans.max(1);
            let end = row[i][j].min(col[i][j]);
            for len in (start..=end).rev() {
                let k = len as usize - 1;
                if row[i + k][j] >= len && col[i][j + k] >= len {
                    ans = len;
                    break;
                }
            }
        }
    }
    ans * ans
}

fn main() {
    {
        let grid = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 9;
        assert_eq!(largest1_bordered_square(grid), ans);
    }

    {
        let grid = [[1, 1, 0, 0]].iter().map(|v| v.to_vec()).collect();
        let ans = 1;
        assert_eq!(largest1_bordered_square(grid), ans);
    }
}
