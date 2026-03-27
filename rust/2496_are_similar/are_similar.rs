struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();
        let k = (k as usize) % n;

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] != mat[i][(j + k) % n] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2,
            true,
        ),
        (vec![vec![2, 2], vec![2, 2]], 3, true),
        (vec![vec![1, 2]], 1, false),
    ];

    for (mat, k, expected) in tests {
        assert_eq!(Solution::are_similar(mat, k), expected);
    }
}
