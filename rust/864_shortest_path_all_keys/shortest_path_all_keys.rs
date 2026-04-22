/*
 * @Date: 2022-11-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-10
 * @FilePath: /algorithm/864_shortest_path_all_keys/shortest_path_all_keys.rs
 */

struct Solution;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let dirs = [-1, 0, 1, 0, -1];
        let mut grid: Vec<_> = grid.into_iter().map(|g| g.into_bytes()).collect();
        let (n, m) = (grid.len(), grid[0].len());
        let (mut start_x, mut start_y) = (0, 0);
        let key_cnt = grid.iter().enumerate().fold(0, |cnt, (i, row)| {
            cnt + row.iter().enumerate().fold(0, |cnt, (j, x)| {
                if *x == b'@' {
                    start_x = i;
                    start_y = j;
                }
                cnt + x.is_ascii_lowercase() as u8
            })
        });
        grid[start_x][start_y] = b'.';
        let mut fifo = std::collections::VecDeque::with_capacity(n * m);
        let mut visited = std::collections::HashSet::with_capacity(n * m);
        fifo.push_back((start_x as i8, start_y as i8, 0u8, 0));
        while let Some((cur_x, cur_y, mask, step)) = fifo.pop_front() {
            if mask == (1 << key_cnt) - 1 {
                return step;
            }
            if !visited.insert((cur_x, cur_y, mask)) {
                continue;
            }
            for (&dx, &dy) in (&dirs[..4]).iter().zip((&dirs[1..]).iter()) {
                let (x, y) = (cur_x + dx, cur_y + dy);
                if x < 0 || x >= n as i8 || y < 0 || y >= m as i8 {
                    continue;
                }
                let t = grid[x as usize][y as usize];
                if t == b'.' {
                    fifo.push_back((x, y, mask, step + 1));
                } else if t.is_ascii_uppercase() {
                    let can_pass = 1 << (t - b'A');
                    if mask & can_pass != 0 {
                        fifo.push_back((x, y, mask, step + 1));
                    }
                } else if t.is_ascii_lowercase() {
                    let key = 1 << (t - b'a');
                    fifo.push_back((x, y, mask | key, step + 1));
                }
            }
        }
        -1
    }
}

fn main() {
    {
        let grid = vec!["@.a.#", "###.#", "b.A.B"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 8;
        assert_eq!(Solution::shortest_path_all_keys(grid), ans);
    }

    {
        let grid = vec!["@..aA", "..B#.", "....b"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 6;
        assert_eq!(Solution::shortest_path_all_keys(grid), ans);
    }

    {
        let grid = vec!["@Aa"].iter().map(|s| s.to_string()).collect();
        let ans = -1;
        assert_eq!(Solution::shortest_path_all_keys(grid), ans);
    }
}
