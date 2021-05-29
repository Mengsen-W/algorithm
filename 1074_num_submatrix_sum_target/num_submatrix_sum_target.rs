/*
 * @Date: 2021-05-29 09:38:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-29 10:13:43
 */

fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    use std::collections::HashMap;
    let m = matrix.len();
    let n = matrix[0].len();
    let mut ans = 0;
    //前缀预处理
    for i in 0..m {
        for j in 1..n {
            matrix[i][j] += matrix[i][j - 1];
        }
    }

    for k in 0..n {
        for j in k..n {
            let mut sum = 0;
            let mut mp: HashMap<i32, i32> = HashMap::new();
            mp.insert(0, 1);
            for i in 0..m {
                if k == 0 {
                    sum += matrix[i][j]
                } else {
                    sum += matrix[i][j] - matrix[i][k - 1];
                }
                if mp.contains_key(&(sum - target)) {
                    ans += *(mp.entry(sum - target).or_insert(0));
                }
                let count = mp.entry(sum).or_insert(0);
                *count += 1;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0),
        4
    );
    assert_eq!(
        num_submatrix_sum_target(vec![vec![-1, 1], vec![1, -1]], 0),
        5
    );
    assert_eq!(num_submatrix_sum_target(vec![vec![904]], 0), 0);
}
