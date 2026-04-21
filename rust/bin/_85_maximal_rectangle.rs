struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut left = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[i][j] = if j == 0 { 0 } else { left[i][j - 1] } + 1;
                }
            }
        }

        let mut ret = 0;
        for j in 0..n {
            let mut up = vec![0; m];
            let mut down = vec![0; m];
            let mut stk: Vec<usize> = Vec::new();

            for i in 0..m {
                while let Some(&top) = stk.last() {
                    if left[top][j] >= left[i][j] {
                        stk.pop();
                    } else {
                        break;
                    }
                }
                up[i] = stk.last().map(|&x| x as i32).unwrap_or(-1);
                stk.push(i);
            }

            stk.clear();
            for i in (0..m).rev() {
                while let Some(&top) = stk.last() {
                    if left[top][j] >= left[i][j] {
                        stk.pop();
                    } else {
                        break;
                    }
                }
                down[i] = stk.last().copied().unwrap_or(m);
                stk.push(i);
            }

            for i in 0..m {
                let height = down[i] - up[i] as usize - 1;
                ret = ret.max(height * left[i][j]);
            }
        }

        ret as i32
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ],
            6,
        ),
        (vec![vec!['0']], 0),
        (vec![vec!['1']], 1),
    ];

    for (matrix, ans) in tests {
        assert_eq!(Solution::maximal_rectangle(matrix), ans);
    }
}
