/*
 * @Date: 2022-09-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-18
 * @FilePath: /algorithm/827_largest_island/largest_island.rs
 */

struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(map: &mut Vec<Vec<i32>>, set: &mut Vec<(i32, i32)>, xx: i32, yy: i32) -> i32 {
            let dir = [-1, 0, 1, 0, -1];
            let mut cnt = 1;
            map[xx as usize][yy as usize] = 2;
            let n = map.len() as i32;
            for i in 0..4 {
                let x = xx + dir[i];
                let y = yy + dir[i + 1];
                if x < 0 || y < 0 || x >= n || y >= n {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if map[x][y] == 1 {
                    cnt += dfs(map, set, x as i32, y as i32);
                } else if map[x][y] == 0 {
                    set.push((x as i32, y as i32));
                    map[x][y] = 3; // 不能重复访问 0
                }
            }
            cnt
        }
        let mut map = grid;
        let n = map.len();
        let total = n * n;
        let mut tab = vec![0; total];
        let mut max_island = 0;
        let mut set = Vec::with_capacity(total);
        for i in 0..map.len() {
            for j in 0..map.len() {
                if map[i][j] == 1 {
                    let cnt = dfs(&mut map, &mut set, i as i32, j as i32);
                    for p in set.iter() {
                        let (x, y) = (p.0 as usize, p.1 as usize);
                        tab[x * n + y] += cnt;
                        map[x][y] = 0; // 将前面 3 修改为 0，确保下一个岛屿能正常访问
                    }
                    max_island = max_island.max(cnt);
                    set.clear();
                }
            }
        }
        (max_island.max(tab.into_iter().max().unwrap() + 1)).min(total as i32)
    }
}

fn main() {
    {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let ans = 3;
        assert_eq!(Solution::largest_island(grid), ans);
    }

    {
        let grid = vec![vec![1, 1], vec![1, 0]];
        let ans = 4;
        assert_eq!(Solution::largest_island(grid), ans);
    }

    {
        let grid = vec![vec![1, 1], vec![1, 1]];
        let ans = 4;
        assert_eq!(Solution::largest_island(grid), ans);
    }
}
