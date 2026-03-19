struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = 0;
        let mut sum = vec![vec![vec![0; 2]; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    'X' => {
                        sum[i + 1][j + 1][0] =
                            sum[i + 1][j][0] + sum[i][j + 1][0] - sum[i][j][0] + 1;
                        sum[i + 1][j + 1][1] = 1;
                    }
                    'Y' => {
                        sum[i + 1][j + 1][0] =
                            sum[i + 1][j][0] + sum[i][j + 1][0] - sum[i][j][0] - 1;
                        sum[i + 1][j + 1][1] = if sum[i + 1][j][1] == 1 || sum[i][j + 1][1] == 1 {
                            1
                        } else {
                            0
                        };
                    }
                    _ => {
                        sum[i + 1][j + 1][0] = sum[i + 1][j][0] + sum[i][j + 1][0] - sum[i][j][0];
                        sum[i + 1][j + 1][1] = if sum[i + 1][j][1] == 1 || sum[i][j + 1][1] == 1 {
                            1
                        } else {
                            0
                        };
                    }
                }

                if sum[i + 1][j + 1][0] == 0 && sum[i + 1][j + 1][1] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']], 3),
        (vec![vec!['X', 'X'], vec!['X', 'Y']], 0),
        (vec![vec!['.', '.'], vec!['.', '.']], 0),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::number_of_submatrices(grid), expected);
    }
}
