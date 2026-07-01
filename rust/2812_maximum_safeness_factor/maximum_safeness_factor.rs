struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let m = grid.len();
        let n = grid[0].len();

        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut dis = vec![vec![-1; n]; m];
        let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut q = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i, j));
                    dis[i][j] = 0;
                }
            }
        }

        while let Some((cx, cy)) = q.pop_front() {
            for &(dx, dy) in &dirs {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if dis[nx][ny] == -1 {
                        dis[nx][ny] = dis[cx][cy] + 1;
                        q.push_back((nx, ny));
                    }
                }
            }
        }

        let check = |limit: i32| -> bool {
            let mut visit = vec![vec![false; n]; m];
            let mut q = VecDeque::new();
            q.push_back((0, 0));
            visit[0][0] = true;

            while let Some((cx, cy)) = q.pop_front() {
                if cx == m - 1 && cy == n - 1 {
                    return true;
                }
                for &(dx, dy) in &dirs {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if !visit[nx][ny] && dis[nx][ny] >= limit {
                            q.push_back((nx, ny));
                            visit[nx][ny] = true;
                        }
                    }
                }
            }
            false
        };

        let mut lo = 0;
        let mut hi = dis[0][0].min(dis[m - 1][n - 1]);
        let mut res = 0;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if check(mid) {
                res = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]], 0),
        (vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]], 2),
        (
            vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            2,
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::maximum_safeness_factor(grid), ans);
    }
}
