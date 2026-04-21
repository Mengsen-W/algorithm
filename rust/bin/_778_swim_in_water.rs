struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = n * n;
        let mut p: Vec<usize> = (0..m).collect();
        let mut hi = vec![0usize; m];

        for i in 0..n {
            for j in 0..n {
                hi[grid[i][j] as usize] = i * n + j;
            }
        }

        fn find(x: usize, p: &mut Vec<usize>) -> usize {
            if p[x] != x {
                p[x] = find(p[x], p);
            }
            p[x]
        }

        let dirs = [-1isize, 0, 1, 0, -1];

        for t in 0..m {
            let id = hi[t];
            let x = id / n;
            let y = id % n;

            for k in 0..4 {
                let nx = x as isize + dirs[k];
                let ny = y as isize + dirs[k + 1];
                if nx >= 0 && nx < n as isize && ny >= 0 && ny < n as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if grid[nx][ny] as usize <= t {
                        let a = find(x * n + y, &mut p);
                        let b = find(nx * n + ny, &mut p);
                        p[a] = b;
                    }
                }
            }

            if find(0, &mut p) == find(m - 1, &mut p) {
                return t as i32;
            }
        }

        0
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 2], vec![1, 3]], 3),
        (
            vec![
                vec![0, 1, 2, 3, 4],
                vec![24, 23, 22, 21, 5],
                vec![12, 13, 14, 15, 16],
                vec![11, 17, 18, 19, 20],
                vec![10, 9, 8, 7, 6],
            ],
            16,
        ),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::swim_in_water(grid), expected);
    }
}
