/*
 * @Date: 2022-01-01 02:00:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-01 03:01:07
 */

pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    match original.len() as i32 == m * n {
        false => Vec::new(),
        true => original
            .chunks(n as usize)
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<i32>>>(),
    }
}

fn main() {
    let non_vec: Vec<Vec<i32>> = vec![];
    assert_eq!(
        construct2_d_array(vec![1, 2, 3, 4], 2, 2),
        vec![vec![1, 2], vec![3, 4]]
    );

    assert_eq!(construct2_d_array(vec![1, 2, 3], 1, 3), vec![vec![1, 2, 3]]);
    assert_eq!(construct2_d_array(vec![1, 2], 1, 1), non_vec);
    assert_eq!(construct2_d_array(vec![3], 1, 2), non_vec);
}
