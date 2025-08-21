struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut f = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    f[i][j] = matrix[i][j];
                } else if matrix[i][j] == 0 {
                    f[i][j] = 0;
                } else {
                    f[i][j] = f[i][j - 1].min(f[i - 1][j]).min(f[i - 1][j - 1]) + 1;
                }
                ans += f[i][j];
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]],
            15,
        ),
        (vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]], 7),
    ];

    for (matrix, expected) in tests {
        assert_eq!(Solution::count_squares(matrix), expected);
    }
}
