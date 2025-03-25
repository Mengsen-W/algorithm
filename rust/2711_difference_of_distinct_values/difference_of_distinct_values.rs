struct Solution;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![vec![0; n]; m];

        for i in 0..m {
            let mut x = i as i32;
            let mut y = 0;
            let mut s = HashSet::new();
            while x < m as i32 && y < n as i32 {
                res[x as usize][y as usize] += s.len() as i32;
                s.insert(grid[x as usize][y as usize]);
                x += 1;
                y += 1;
            }
        }

        for j in 1..n {
            let mut x = 0;
            let mut y = j as i32;
            let mut s = HashSet::new();
            while x < m as i32 && y < n as i32 {
                res[x as usize][y as usize] += s.len() as i32;
                s.insert(grid[x as usize][y as usize]);
                x += 1;
                y += 1;
            }
        }

        for i in 0..m {
            let mut x = i as i32;
            let mut y = n as i32 - 1;
            let mut s = HashSet::new();
            while x >= 0 && y >= 0 {
                res[x as usize][y as usize] -= s.len() as i32;
                res[x as usize][y as usize] = (res[x as usize][y as usize]).abs();
                s.insert(grid[x as usize][y as usize]);
                x -= 1;
                y -= 1;
            }
        }

        for j in (0..n - 1).rev() {
            let mut x = m as i32 - 1;
            let mut y = j as i32;
            let mut s = HashSet::new();
            while x >= 0 && y >= 0 {
                res[x as usize][y as usize] -= s.len() as i32;
                res[x as usize][y as usize] = (res[x as usize][y as usize]).abs();
                s.insert(grid[x as usize][y as usize]);
                x -= 1;
                y -= 1;
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]],
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]],
        ),
        (vec![vec![1]], vec![vec![0]]),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::difference_of_distinct_values(grid), ans);
    }
}
