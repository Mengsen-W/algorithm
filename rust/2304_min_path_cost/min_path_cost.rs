/*
 * @Date: 2023-11-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-22
 * @FilePath: /algorithm/rust/2304_min_path_cost/min_path_cost.rs
 */

struct Solution;

impl Solution {
    pub fn min_path_cost(mut grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for i in (0..m - 1).rev() {
            for j in 0..n {
                grid[i][j] += grid[i + 1]
                    .iter()
                    .zip(move_cost[grid[i][j] as usize].iter())
                    .map(|(&g, &c)| g + c)
                    .min()
                    .unwrap();
            }
        }
        *grid[0].iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![5, 3], vec![4, 0], vec![2, 1]],
            vec![
                vec![9, 8],
                vec![1, 5],
                vec![10, 12],
                vec![18, 6],
                vec![2, 4],
                vec![14, 3],
            ],
            17,
        ),
        (
            vec![vec![5, 1, 2], vec![4, 0, 3]],
            vec![
                vec![12, 10, 15],
                vec![20, 23, 8],
                vec![21, 7, 1],
                vec![8, 1, 13],
                vec![9, 10, 25],
                vec![5, 3, 2],
            ],
            6,
        ),
    ];

    for (grid, move_cost, ans) in tests {
        assert_eq!(Solution::min_path_cost(grid, move_cost), ans);
    }
}
