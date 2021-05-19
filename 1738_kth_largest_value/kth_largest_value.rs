/*
 * @Date: 2021-05-19 08:47:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-19 09:09:13
 */
fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix.get(0).unwrap().len();
    let mut pre = vec![vec![0; n + 1]; m + 1];
    let mut result = Vec::new();
    for i in 1..=m {
        for j in 1..=n {
            pre[i][j] = pre[i - 1][j] ^ pre[i][j - 1] ^ pre[i - 1][j - 1] ^ matrix[i - 1][j - 1];
            result.push(pre[i][j]);
        }
    }
    result.sort_by(|a, b| b.partial_cmp(a).unwrap());
    return result[k as usize - 1];
}

fn main() {
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2), 5);
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3), 4);
    assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 4), 0);
}
