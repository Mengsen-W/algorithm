/*
 * @Date: 2023-08-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-11
 * @FilePath: /algorithm/rust/1572_diagonal_sum/diagonal_sum.rs
 */

struct Solution;
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        // Passed 0ms 2.1mb
        let (mut begin, mut end, mut sum) = (0, mat.len() - 1, 0);
        while begin < end {
            sum += mat[begin][begin] + mat[end][begin] + mat[begin][end] + mat[end][end];
            begin += 1;
            end -= 1;
        }
        if begin == end {
            sum += mat[begin][begin];
        }
        sum
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 25),
        (
            vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ],
            8,
        ),
        (vec![vec![5]], 5),
    ];

    for (mat, ans) in tests {
        assert_eq!(Solution::diagonal_sum(mat), ans);
    }
}
