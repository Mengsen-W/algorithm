struct Solution;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut ans = 0;

        for i in 1..n {
            for j in 1..m {
                if grid[i][j] == 0 {
                    if Self::dfs(&mut grid, n, m, i, j) {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, n: usize, m: usize, i: usize, j: usize) -> bool {
        if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
            return false;
        }

        let mut is_closed = true;
        grid[i as usize][j as usize] = 1;

        for (di, dj) in Self::DIRS {
            let ni = (i as i32 + di) as usize;
            let nj = (j as i32 + dj) as usize;

            if ni < n && nj < m && grid[ni][nj] == 0 {
                is_closed &= Self::dfs(grid, n, m, ni, nj);
            }
        }

        is_closed
    }
}

fn main() {
    let tests = vec![
        (
            [
                [1, 1, 1, 1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1, 1, 0],
                [1, 0, 1, 0, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 0],
            ]
            .iter()
            .map(|v| v.to_vec())
            .collect(),
            2,
        ),
        (
            [[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            1,
        ),
        (
            [
                [1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 0, 1, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1],
            ]
            .iter()
            .map(|v| v.to_vec())
            .collect(),
            2,
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::closed_island(grid), ans);
    }
}
