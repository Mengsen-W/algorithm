/*
 * @Date: 2024-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-04
 * @FilePath: /algorithm/rust/2397_maximum_rows/maximum_rows.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let bits_len = matrix[0].len();
        let tab: Vec<_> = matrix
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .enumerate()
                    .fold(0, |x, (i, b)| (x | (b << i) as u32))
            })
            .collect();
        let mut ans = 0;
        for x in 0u32..1 << bits_len {
            if x.count_ones() != num_select as u32 {
                continue;
            }
            let cnt = tab.iter().fold(0, |cnt, &a| cnt + ((a & x) == a) as i32);
            ans = ans.max(cnt);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
            2,
            3,
        ),
        (vec![vec![1], vec![0]], 1, 2),
    ];

    for (matrix, num_select, ans) in tests {
        assert_eq!(Solution::maximum_rows(matrix, num_select), ans);
    }
}
