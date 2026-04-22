struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut f = vec![0; n];
        f[0] = triangle[0][0];

        for i in 1..n {
            f[i] = f[i - 1] + triangle[i][i];
            for j in (1..i).rev() {
                f[j] = std::cmp::min(f[j - 1], f[j]) + triangle[i][j];
            }
            f[0] += triangle[i][0];
        }

        *f.iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
            11,
        ),
        (vec![vec![-10]], -10),
    ];

    for (triangle, ans) in tests {
        assert_eq!(Solution::minimum_total(triangle), ans);
    }
}
