struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![vec![0; n - k + 1]; m - k + 1];
        for i in 0..=m - k {
            for j in 0..=n - k {
                let mut kgrid = Vec::new();
                for x in i..i + k {
                    for y in j..j + k {
                        kgrid.push(grid[x][y]);
                    }
                }
                let mut kmin = i32::MAX;
                kgrid.sort();
                for t in 1..kgrid.len() {
                    if kgrid[t] == kgrid[t - 1] {
                        continue;
                    }
                    kmin = std::cmp::min(kmin, kgrid[t] - kgrid[t - 1]);
                }
                if kmin != i32::MAX {
                    res[i][j] = kmin;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 8], vec![3, -2]], 2, vec![vec![2]]),
        (vec![vec![3, -1]], 1, vec![vec![0, 0]]),
        (vec![vec![1, -2, 3], vec![2, 3, 5]], 2, vec![vec![1, 2]]),
    ];

    for (grid, k, expected) in tests {
        assert_eq!(Solution::min_abs_diff(grid, k), expected);
    }
}
