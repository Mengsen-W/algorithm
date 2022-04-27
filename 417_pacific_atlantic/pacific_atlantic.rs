/*
 * @Date: 2022-04-27 09:41:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-27 10:08:31
 * @FilePath: /algorithm/417_pacific_atlantic/pacific_atlantic.rs
 */

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn bfs(heights: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if visited[i][j] {
            return;
        }
        visited[i][j] = true;
        let mut queue = vec![(i, j)];
        while let Some((i, j)) = queue.pop() {
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni < 0
                    || ni >= heights.len() as i32
                    || nj < 0
                    || nj >= heights[0].len() as i32
                    || visited[ni as usize][nj as usize]
                    || heights[ni as usize][nj as usize] < heights[i][j]
                {
                    continue;
                }
                visited[ni as usize][nj as usize] = true;
                queue.push((ni as usize, nj as usize));
            }
        }
    }
    let mut visited_p = vec![vec![false; heights[0].len()]; heights.len()];
    let mut visited_a = vec![vec![false; heights[0].len()]; heights.len()];
    let mut res = vec![];
    for i in 0..heights.len() {
        bfs(&heights, &mut visited_p, i, 0);
        bfs(&heights, &mut visited_a, i, heights[0].len() - 1);
    }
    for i in 0..heights[0].len() {
        bfs(&heights, &mut visited_p, 0, i);
        bfs(&heights, &mut visited_a, heights.len() - 1, i);
    }
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            if visited_p[i][j] && visited_a[i][j] {
                res.push(vec![i as i32, j as i32]);
            }
        }
    }
    res
}

fn main() {
    {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let ans = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        assert_eq!(pacific_atlantic(heights), ans);
    }

    {
        let heights = vec![vec![2, 1], vec![1, 2]];
        let ans = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        assert_eq!(pacific_atlantic(heights), ans);
    }
}
