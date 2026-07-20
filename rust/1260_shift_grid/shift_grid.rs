struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut a: Vec<i32> = grid.into_iter().flatten().collect();
        a.rotate_right(k as usize % (m * n));
        a.chunks(n).map(|v| v.to_vec()).collect()
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            1,
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
        ),
        (
            vec![
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
                vec![12, 0, 21, 13],
            ],
            4,
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
            ],
        ),
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            9,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        ),
    ];

    for (grid, k, expected) in tests {
        assert_eq!(Solution::shift_grid(grid, k), expected);
    }
}
