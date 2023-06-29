/*
 * @Date: 2023-06-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-29
 * @FilePath: /algorithm/rust/1253_reconstruct_matrix/reconstruct_matrix.rs
 */

pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = colsum.len();
    let mut ans = vec![vec![0; n]; 2];

    for j in 0..n {
        match colsum[j] {
            2 => {
                ans[0][j] = 1;
                ans[1][j] = 1;
                upper -= 1;
                lower -= 1;
            }
            1 => {
                if upper >= lower {
                    ans[0][j] = 1;
                    upper -= 1;
                } else {
                    ans[1][j] = 1;
                    lower -= 1;
                }
            }
            _ => {}
        }
    }

    if upper == 0 && lower == 0 {
        ans
    } else {
        vec![]
    }
}

fn main() {
    let test_map = vec![
        (2, 1, vec![1, 1, 1], vec![vec![1, 1, 0], vec![0, 0, 1]]),
        (2, 3, vec![2, 2, 1, 1], vec![]),
        (
            5,
            5,
            vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1],
            vec![
                vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
                vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
            ],
        ),
    ];

    for item in test_map {
        assert_eq!(reconstruct_matrix(item.0, item.1, item.2), item.3)
    }
}
