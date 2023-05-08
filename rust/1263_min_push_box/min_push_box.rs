/*
 * @Date: 2023-05-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-08
 * @FilePath: /algorithm/rust/1263_min_push_box/min_push_box.rs
 */

pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
    use std::collections::HashMap;
    use std::collections::VecDeque;
    let mut hash: HashMap<((usize, usize), (usize, usize)), i32> = HashMap::new();
    let mut q = VecDeque::new();
    let mut b_pos = (0, 0);
    let mut s_pos = (0, 0);
    let mut t_pos = (0, 0);
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'B' {
                b_pos = (i, j);
            } else if grid[i][j] == 'S' {
                s_pos = (i, j);
            } else if grid[i][j] == 'T' {
                t_pos = (i, j);
            }
        }
    }
    if b_pos == t_pos {
        return 0;
    }

    // 判断人是否可以从s_pos到达new_pos
    fn accesible(grid: &Vec<Vec<char>>, s_pos: &(usize, usize), new_pos: &(usize, usize)) -> bool {
        if s_pos == new_pos {
            return true;
        }
        let mut q = VecDeque::new();
        let mut hash = HashMap::new();
        let s_pos = s_pos.clone();
        q.push_back(s_pos);
        let to: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
            for i in 0..4 {
                let pos = (
                    pos.0.wrapping_add(to[i].0 as usize),
                    pos.1 + to[i].1 as usize,
                );
                if pos.0 >= grid.len() || pos.1 >= grid[0].len() {
                    continue;
                }
                if grid[pos.0][pos.1] == '#' || grid[pos.0][pos.1] == 'B' {
                    continue;
                }
                if &pos == new_pos {
                    return true;
                }
                if hash.contains_key(&pos) {
                    continue;
                }
                hash.insert(pos.clone(), true);
                q.push_back(pos);
            }
        }
        false
    }

    // left, right, up, down
    let to: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let from: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut ans = -1;
    q.push_back((s_pos, b_pos, 0, grid, vec![b_pos.clone()]));
    while !q.is_empty() {
        let (s_pos, b_pos, step, grid, path) = q.pop_front().unwrap();
        for i in 0..to.len() {
            let new_s_pos = (
                b_pos.0.wrapping_add(from[i].0 as usize),
                b_pos.1.wrapping_add(from[i].1 as usize),
            );
            let new_b_pos = (
                b_pos.0.wrapping_add(to[i].0 as usize),
                b_pos.1.wrapping_add(to[i].1 as usize),
            );

            if new_s_pos.0 >= m || new_s_pos.1 >= n {
                continue;
            }
            if new_b_pos.0 >= m || new_b_pos.1 >= n {
                continue;
            }

            if grid[new_s_pos.0][new_s_pos.1] != '#' && grid[new_b_pos.0][new_b_pos.1] != '#' {
                let mut new_grid = grid.clone();
                new_grid[b_pos.0][b_pos.1] = '.';
                new_grid[s_pos.0][s_pos.1] = '.';
                new_grid[new_b_pos.0][new_b_pos.1] = 'B';
                new_grid[new_s_pos.0][new_s_pos.1] = 'S';

                if hash.contains_key(&(new_s_pos, new_b_pos)) {
                    continue;
                }
                if !accesible(&grid, &s_pos, &new_s_pos) {
                    continue;
                }
                if new_b_pos == t_pos {
                    println!("path {:?}", path);
                    ans = step + 1;
                    q.clear();
                    break;
                }

                hash.insert((new_s_pos.clone(), new_b_pos.clone()), step + 1);
                let mut path = path.clone();
                path.push(new_b_pos.clone());
                q.push_back((new_s_pos, new_b_pos, step + 1, new_grid, path));
            }
        }
    }

    ans
}

fn main() {
    {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];
        let ans = 3;
        assert_eq!(min_push_box(grid), ans);
    }

    {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];
        let ans = -1;
        assert_eq!(min_push_box(grid), ans);
    }

    {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '.', '.', '#', '#'],
            vec!['#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];
        let ans = 5;
        assert_eq!(min_push_box(grid), ans);
    }
}
