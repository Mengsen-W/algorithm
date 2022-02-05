/*
 * @Date: 2022-02-05 02:08:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-05 03:36:27
 */

pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = 0;
    fn dfs(
        x: i32,
        y: i32,
        mut gold: i32,
        grid: &mut Vec<Vec<i32>>,
        ans: &mut i32,
        m: &usize,
        n: &usize,
    ) {
        gold += grid[x as usize][y as usize];
        *ans = *ans.max(&mut gold);
        let rec = grid[x as usize][y as usize];
        grid[x as usize][y as usize] = 0;

        for d in 0..4 {
            let nx = x + DIRS[d].0;
            let ny = y + DIRS[d].1;
            if nx >= 0
                && nx < *m as i32
                && ny >= 0
                && ny < *n as i32
                && grid[nx as usize][ny as usize] > 0
            {
                dfs(nx, ny, gold, grid, ans, m, n);
            }
        }
        grid[x as usize][y as usize] = rec;
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 0 {
                dfs(i as i32, j as i32, 0, &mut grid, &mut ans, &m, &n);
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
        24
    );

    assert_eq!(
        get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20]
        ]),
        28
    );
}
