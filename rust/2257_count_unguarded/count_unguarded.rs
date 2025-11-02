struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0; n]; m]; // 网格状态数组
        let mut q = VecDeque::new(); // 广度优先搜索队列
                                     // 每个方向的单位向量
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        for guard in guards {
            let (x, y) = (guard[0] as usize, guard[1] as usize);
            grid[x][y] = -1;
            for k in 0..4 {
                // 将四个方向视线对应的状态均添加进搜索队列中
                q.push_back((x, y, k));
            }
        }
        for wall in walls {
            let (x, y) = (wall[0] as usize, wall[1] as usize);
            grid[x][y] = -2;
        }
        while let Some((x, y, k)) = q.pop_front() {
            let nx = x as i32 + dx[k];
            let ny = y as i32 + dy[k];
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] >= 0 {
                    // 沿着视线方向的下一个坐标合法，且不为警卫或墙
                    if (grid[nx][ny] & (1 << k)) == 0 {
                        // 对应状态未遍历过
                        grid[nx][ny] |= 1 << k;
                        q.push_back((nx, ny, k));
                    }
                }
            }
        }
        let mut res = 0; // 未被保护格子数目
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]],
            7,
        ),
        (
            3,
            3,
            vec![vec![1, 1]],
            vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
            4,
        ),
    ];
    for (m, n, guards, walls, ans) in tests {
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), ans);
    }
}
