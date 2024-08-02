struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut col = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                col[j] += grid[i][j];
            }
        }
        let mut res: i64 = 0;
        for i in 0..n {
            let row: i32 = grid[i].iter().sum();
            for j in 0..m {
                if grid[i][j] == 1 {
                    res += (row - 1) as i64 * (col[j] - 1) as i64;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]], 2),
        (
            vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]],
            0,
        ),
        (vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]], 2),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::number_of_right_triangles(grid), ans);
    }
}
