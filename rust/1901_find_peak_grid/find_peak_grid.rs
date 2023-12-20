/*
 * @Date: 2023-12-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-20
 * @FilePath: /algorithm/rust/1901_find_peak_grid/find_peak_grid.rs
 */

struct Solution;

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = mat.len() - 1;
        while l < r {
            let mid: usize = (l + r) >> 1;
            let j: usize = mat[mid]
                .iter()
                .position(|&x| x == *mat[mid].iter().max().unwrap())
                .unwrap();
            if mat[mid][j] > mat[mid + 1][j] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        let j: usize = mat[l]
            .iter()
            .position(|&x| x == *mat[l].iter().max().unwrap())
            .unwrap();
        vec![l as i32, j as i32]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 4], vec![3, 2]], vec![0, 1]),
        (
            vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]],
            vec![1, 1],
        ),
    ];

    for (mat, ans) in tests {
        assert_eq!(Solution::find_peak_grid(mat), ans);
    }
}
