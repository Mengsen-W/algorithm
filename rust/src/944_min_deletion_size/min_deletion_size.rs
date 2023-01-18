/*
 * @Date: 2022-05-12 09:30:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-12 09:50:37
 * @FilePath: /algorithm/944_min_deletion_size/min_deletion_size.rs
 */

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ret = 0;
    let strs_arr = strs
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    for i in 0..strs[0].len() {
        for j in 1..strs.len() {
            if strs_arr[j - 1][i] > strs_arr[j][i] {
                ret += 1;
                break;
            }
        }
    }
    ret
}

fn main() {
    assert_eq!(
        min_deletion_size(
            vec!["cba", "daf", "ghi"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        1
    );
    assert_eq!(
        min_deletion_size(vec!["a", "b"].iter().map(|s| s.to_string()).collect()),
        0
    );

    assert_eq!(
        min_deletion_size(
            vec!["zyx", "wvu", "tsr"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        3
    );
}
