struct Solution;

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let size = 2 * n + 1;
        let mut count = vec![vec![0; size]; size];

        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 {
                    for i2 in 0..n {
                        for j2 in 0..n {
                            if b[i2][j2] == 1 {
                                count[i - i2 + n][j - j2 + n] += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for row in &count {
            for &v in row {
                ans = ans.max(v);
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
            3,
        ),
        (vec![vec![1]], vec![vec![1]], 1),
        (vec![vec![0]], vec![vec![0]], 0),
    ];

    for (a, b, expected) in tests {
        assert_eq!(Solution::largest_overlap(a, b), expected);
    }
}
