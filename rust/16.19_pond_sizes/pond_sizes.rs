/*
 * @Date: 2023-06-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-22
 * @FilePath: /algorithm/rust/16.19_pond_sizes/pond_sizes.rs
 */

struct Solution;

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let m = land.len();
        let n = land[0].len();
        let mut res = vec![];
        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 0 {
                    res.push(Self::dfs(&mut land, i, j));
                }
            }
        }
        res.sort_unstable();
        return res;
    }

    fn dfs(land: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        let m = land.len();
        let n = land[0].len();
        land[x][y] = 1;
        let mut size = 1;

        // 上下左右 + 斜线
        const DIRS: &[(i32, i32); 8] = &[
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        for i in 0..8 {
            let nx = x as i32 + DIRS[i].0;
            let ny = y as i32 + DIRS[i].1;
            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                continue;
            }
            if land[nx as usize][ny as usize] == 0 {
                size += Self::dfs(land, nx as usize, ny as usize);
            }
        }

        return size;
    }
}

fn main() {
    let land = vec![
        vec![0, 2, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
    ];
    let ans = vec![1, 2, 4];
    assert_eq!(Solution::pond_sizes(land), ans);
}
