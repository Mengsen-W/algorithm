/*
 * @Date: 2023-03-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-01
 * @FilePath: /algorithm/rust/2373_largest_local/largest_local.rs
 */

pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut ans = vec![];

    for i in 1..n - 1 {
        let mut tmp = vec![];

        for j in 1..n - 1 {
            let largest = grid[i - 1][j - 1]
                .max(grid[i - 1][j])
                .max(grid[i - 1][j + 1])
                .max(grid[i][j - 1])
                .max(grid[i][j])
                .max(grid[i][j + 1])
                .max(grid[i + 1][j - 1])
                .max(grid[i + 1][j])
                .max(grid[i + 1][j + 1]);

            tmp.push(largest);
        }

        ans.push(tmp);
    }
    ans
}

fn main() {
    {
        let grid = [[9, 9, 8, 1], [5, 6, 2, 6], [8, 2, 6, 4], [6, 2, 2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = [[9, 9], [8, 6]].iter().map(|v| v.to_vec()).collect();
        assert_eq!(largest_local(grid), ans);
    }

    {
        let grid = [
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 2, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect();
        let ans: Vec<Vec<i32>> = [[2, 2, 2], [2, 2, 2], [2, 2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(largest_local(grid), ans);
    }
}
