struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        let mut mat = mat;

        // 最多旋转 4 次
        for _ in 0..4 {
            // 旋转操作
            for i in 0..n / 2 {
                for j in 0..(n + 1) / 2 {
                    let temp = mat[i][j];
                    mat[i][j] = mat[n - 1 - j][i];
                    mat[n - 1 - j][i] = mat[n - 1 - i][n - 1 - j];
                    mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
                    mat[j][n - 1 - i] = temp;
                }
            }

            if mat == target {
                return true;
            }
        }
        false
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]],
            true,
        ),
        (
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]],
            false,
        ),
        (
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]],
            true,
        ),
    ];

    for (mat, target, expected) in tests {
        assert_eq!(Solution::find_rotation(mat, target), expected);
    }
}
