struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];
        for q in queries.iter() {
            let (row1, col1, row2, col2) =
                (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            diff[row1][col1] += 1;
            diff[row2 + 1][col1] -= 1;
            diff[row1][col2 + 1] -= 1;
            diff[row2 + 1][col2 + 1] += 1;
        }

        let mut mat = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let x1 = if i == 0 { 0 } else { mat[i - 1][j] };
                let x2 = if j == 0 { 0 } else { mat[i][j - 1] };
                let x3 = if i == 0 || j == 0 {
                    0
                } else {
                    mat[i - 1][j - 1]
                };
                mat[i][j] = diff[i][j] + x1 + x2 - x3;
            }
        }
        mat
    }
}

fn main() {
    let tests = vec![
        (
            3,
            vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]],
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]],
        ),
        (2, vec![vec![0, 0, 1, 1]], vec![vec![1, 1], vec![1, 1]]),
    ];
    for (n, queries, expected) in tests {
        assert_eq!(Solution::range_add_queries(n, queries), expected);
    }
}
