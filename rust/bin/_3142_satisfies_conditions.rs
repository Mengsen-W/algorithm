struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i + 1 < grid.len() && grid[i][j] != grid[i + 1][j] {
                    return false;
                }
                if j + 1 < grid[0].len() && grid[i][j] == grid[i][j + 1] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 0, 2], vec![1, 0, 2]], true),
        (vec![vec![1, 1, 1], vec![0, 0, 0]], false),
        (vec![vec![1], vec![2], vec![3]], false),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::satisfies_conditions(grid), ans);
    }
}
