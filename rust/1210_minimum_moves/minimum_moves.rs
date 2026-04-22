/*
 * @Date: 2023-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-05
 * @FilePath: /algorithm/rust/1210_minimum_moves/minimum_moves.rs
 */

pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut q = vec![(0, 0, 1)];
    let mut vis = vec![vec![0; n]; n];
    vis[0][0] |= 1;
    let mut ans = 0;
    fn aux(nx: &mut Vec<(usize, usize, i32)>, vis: &mut Vec<Vec<i32>>, x: usize, y: usize, t: i32) {
        if vis[x][y] & t == 0 {
            nx.push((x, y, t));
            vis[x][y] |= t;
        }
    }
    while !q.is_empty() {
        let mut nx = vec![];
        for (x, y, t) in q {
            if t == 1 {
                let ry = y + 2;
                if ry < n && grid[x][ry] == 0 {
                    aux(&mut nx, &mut vis, x, y + 1, t);
                }
                let rx = x + 1;
                if rx < n && grid[rx][y] == 0 && grid[rx][y + 1] == 0 {
                    aux(&mut nx, &mut vis, x, y, 3 - t);
                    aux(&mut nx, &mut vis, rx, y, t);
                }
            } else {
                let rx = x + 2;
                if rx < n && grid[rx][y] == 0 {
                    aux(&mut nx, &mut vis, x + 1, y, t);
                }
                let ry = y + 1;
                if ry < n && grid[x][ry] == 0 && grid[x + 1][ry] == 0 {
                    aux(&mut nx, &mut vis, x, y, 3 - t);
                    aux(&mut nx, &mut vis, x, ry, t);
                }
            }
        }
        q = nx;
        ans += 1;
        if vis[n - 1][n - 2] & 1 > 0 {
            return ans;
        }
    }
    -1
}

fn main() {
    {
        let grid = [
            [0, 0, 0, 0, 0, 1],
            [1, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 1],
            [0, 0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0],
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect();
        let ans = 11;
        assert_eq!(minimum_moves(grid), ans);
    }

    {
        let grid = [
            [0, 0, 1, 1, 1, 1],
            [0, 0, 0, 0, 1, 1],
            [1, 1, 0, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 0],
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect();
        let ans = 9;
        assert_eq!(minimum_moves(grid), ans);
    }
}
