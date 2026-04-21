/*
 * @Date: 2023-05-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-26
 * @FilePath: /algorithm/rust/1091_shortest_path_binary_matrix/shortest_path_binary_matrix.rs
 */

pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] == 1 {
        return -1;
    }

    let n = grid.len();
    let directions = vec![
        (1, 1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0, 0));
    grid[0][0] = 1;

    while let Some((x, y)) = queue.pop_front() {
        if x == n - 1 && y == n - 1 {
            return grid[x][y];
        }

        for (dx, dy) in directions.iter() {
            let i = (x as i32 + dx) as usize;
            let j = (y as i32 + dy) as usize;

            if i < n && j < n && grid[i][j] == 0 {
                queue.push_back((i, j));
                grid[i][j] = grid[x][y] + 1;
            }
        }
    }

    -1
}

fn main() {
    {
        let grid = [[0, 1], [1, 0]].iter().map(|v| v.to_vec()).collect();
        let ans = 2;
        assert_eq!(shortest_path_binary_matrix(grid), ans);
    }

    {
        let grid = [[0, 0, 0], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 4;
        assert_eq!(shortest_path_binary_matrix(grid), ans);
    }

    {
        let grid = [[1, 0, 0], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = -1;
        assert_eq!(shortest_path_binary_matrix(grid), ans);
    }
}
