/*
 * @Date: 2023-06-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-15
 * @FilePath: /algorithm/rust/1777_can_make_pali_queries/can_make_pali_queries.rs
 */

pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let s = s.into_bytes();
    let mut prefix = Vec::with_capacity(s.len() + 1);
    prefix.push([0; 26]);
    for c in s.into_iter() {
        prefix.push(*prefix.last().unwrap());
        prefix.last_mut().unwrap()[(c - b'a') as usize] += 1;
    }
    queries
        .into_iter()
        .map(|q| {
            let cnt = (0..26).fold(0, |cnt, i| {
                cnt + ((prefix[q[1] as usize + 1][i] - prefix[q[0] as usize][i]) & 1)
            });
            (cnt >> 1) <= q[2]
        })
        .collect()
}

fn main() {
    let s = "abcda".to_string();
    let queries = [[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    let ans = vec![true, false, false, true, true];
    assert_eq!(can_make_pali_queries(s, queries), ans);
}
