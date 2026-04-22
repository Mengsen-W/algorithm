/*
 * @Date: 2021-07-26 10:38:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-26 14:27:45
 */
use std::collections::HashMap;

pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
    let size = target.len();
    let table = target
        .into_iter()
        .enumerate()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    (size
        - arr
            .into_iter()
            .filter_map(|e| table.get(&e).cloned())
            .fold(Vec::new(), |mut acc: Vec<usize>, e| {
                if acc.is_empty() || e > acc.last().cloned().unwrap() {
                    acc.push(e);
                } else if let Err(p) = acc.binary_search(&e) {
                    acc[p] = e;
                }
                acc
            })
            .len()) as i32
}

fn main() {
    {
        let target = vec![5, 1, 3];
        let arr = vec![9, 4, 2, 3, 4];
        assert_eq!(min_operations(target, arr), 2);
    }
    {
        let target = vec![6, 4, 8, 1, 3, 2];
        let arr = vec![4, 7, 6, 2, 3, 8, 6, 1];
        assert_eq!(min_operations(target, arr), 3);
    }
}
