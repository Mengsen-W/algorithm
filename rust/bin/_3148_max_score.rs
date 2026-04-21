struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut precol = vec![i32::MIN; n];
        let mut ans = i32::MIN;

        for i in 0..m {
            let mut prerow = i32::MIN;
            for j in 0..n {
                let mut f = i32::MIN;
                if i > 0 {
                    f = f.max(grid[i][j] + precol[j]);
                }
                if j > 0 {
                    f = f.max(grid[i][j] + prerow);
                }
                ans = ans.max(f);
                precol[j] = precol[j].max(f.max(0) - grid[i][j]);
                prerow = prerow.max(f.max(0) - grid[i][j]);
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![9, 5, 7, 3],
                vec![8, 9, 6, 1],
                vec![6, 7, 14, 3],
                vec![2, 5, 3, 1],
            ],
            9,
        ),
        (vec![vec![4, 3, 2], vec![3, 2, 1]], -1),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::max_score(grid), ans);
    }
}
