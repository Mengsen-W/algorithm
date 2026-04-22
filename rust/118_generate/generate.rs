struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut c = vec![vec![]; n];
        for i in 0..n {
            c[i].resize(i + 1, 1);
            for j in 1..i {
                // 左上方的数 + 正上方的数
                c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
            }
        }
        c
    }
}

fn main() {
    let tests = vec![
        (
            5,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
        ),
        (1, vec![vec![1]]),
    ];

    for (num_rows, expected) in tests {
        assert_eq!(Solution::generate(num_rows), expected);
    }
}
