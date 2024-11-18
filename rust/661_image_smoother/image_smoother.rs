/*
 * @Date: 2022-03-23 23:29:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 23:59:33
 * @FilePath: /algorithm/661_image_smoother/image_smoother.rs
 */

struct Solution;

impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; m[0].len()]; m.len()];

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                let mut cnt = 0;

                for k in i.saturating_sub(1)..=(i + 1).min(m.len() - 1) {
                    for l in j.saturating_sub(1)..=(j + 1).min(m[0].len() - 1) {
                        ret[i][j] += m[k][l];
                        cnt += 1;
                    }
                }

                ret[i][j] /= cnt;
            }
        }
        ret
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        ),
        (
            vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]],
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137],
            ],
        ),
    ];

    for (m, ans) in tests {
        assert_eq!(Solution::image_smoother(m), ans);
    }
}
