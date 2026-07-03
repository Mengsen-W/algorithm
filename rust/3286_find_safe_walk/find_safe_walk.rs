struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        use std::collections::VecDeque;
        let (m, n) = (grid.len(), grid[0].len());
        let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut dis = vec![vec![i32::MAX; n]; m];
        let mut q = VecDeque::new();

        q.push_front((0usize, 0usize));
        dis[0][0] = grid[0][0];

        while let Some((cx, cy)) = q.pop_front() {
            // 第一次出队时，保证是最短距离
            if cx == m - 1 && cy == n - 1 {
                return true;
            }

            for (dx, dy) in dirs.iter() {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx < 0 || ny < 0 || nx >= m as i32 || ny >= n as i32 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                let cost = dis[cx][cy] + grid[nx][ny];
                // 剪枝：新距离不满足健康要求
                if cost >= health {
                    continue;
                }

                if cost < dis[nx][ny] {
                    dis[nx][ny] = cost;
                    if grid[nx][ny] == 0 {
                        q.push_front((nx, ny));
                    } else {
                        q.push_back((nx, ny));
                    }
                }
            }
        }

        false
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]], 1, true),
        (vec![vec![0, 1, 1, 0, 0, 0], vec![1, 0, 1, 0, 0, 0], vec![0, 1, 1, 1, 0, 1], vec![0, 0, 1, 0, 1, 0]], 3, false),
        (vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 5, true),
    ];

    for (grid, health, ans) in tests {
        assert_eq!(Solution::find_safe_walk(grid, health), ans);
    }
}
