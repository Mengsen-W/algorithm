struct Solution;

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        let mut grid = grid;
        let mut i0 = x;
        let mut i1 = x + k - 1;

        while i0 < i1 {
            for j in y..y + k {
                let temp = grid[i0][j];
                grid[i0][j] = grid[i1][j];
                grid[i1][j] = temp;
            }
            i0 += 1;
            i1 -= 1;
        }

        grid
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            1,
            0,
            3,
            vec![
                vec![1, 2, 3, 4],
                vec![13, 14, 15, 8],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 16],
            ],
        ),
        (
            vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]],
            0,
            2,
            2,
            vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]],
        ),
    ];

    for (grid, x, y, k, expected) in tests {
        assert_eq!(Solution::reverse_submatrix(grid, x, y, k), expected);
    }
}
