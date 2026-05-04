struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                // 遍历对角线上方元素，做转置
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }
            matrix[i].reverse(); // 行翻转
        }
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
        ),
        (
            vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ],
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ],
        ),
    ];

    for (mut matrix, expected) in tests {
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
