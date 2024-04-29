/*
 * @Date: 2024-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-29
 * @FilePath: /algorithm/rust/1329_diagonal_sort/diagonal_sort.rs
 */

struct Solution;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;
        let mut a = vec![0; m.min(n) as usize];
        for k in 1 - n..m {
            // k = i - j
            let left_i = k.max(0) as usize;
            let right_i = (k + n).min(m) as usize;
            for i in left_i..right_i {
                a[i - left_i] = mat[i][(i as i32 - k) as usize];
            }
            a[..right_i - left_i].sort_unstable();
            for i in left_i..right_i {
                mat[i][(i as i32 - k) as usize] = a[i - left_i];
            }
        }
        mat
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]],
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]],
        ),
        (
            vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50],
            ],
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68],
            ],
        ),
    ];

    for (mat, ans) in tests {
        assert_eq!(Solution::diagonal_sort(mat), ans);
    }
}
