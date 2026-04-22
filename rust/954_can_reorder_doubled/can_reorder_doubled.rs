/*
 * @Date: 2022-03-31 22:14:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 22:56:55
 * @FilePath: /algorithm/954_can_reorder_doubled/can_reorder_doubled.rs
 */

pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut cnt = arr.iter().fold(HashMap::new(), |mut map, &x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });
    if cnt.get(&0).unwrap_or(&0) % 2 != 0 {
        return false;
    }

    let mut vals = cnt.iter().fold(vec![], |mut vals, (k, _)| {
        vals.push(*k);
        vals
    });
    vals.sort_unstable_by(|a, b| a.abs().cmp(&b.abs()));

    for x in vals {
        if *cnt.get(&(2 * x)).unwrap_or(&0) < cnt[&x] {
            return false;
        }
        *cnt.get_mut(&(2 * x)).unwrap_or(&mut 0) -= cnt[&x];
    }
    true
}

fn main() {
    assert_eq!(can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(can_reorder_doubled(vec![2, 1, 2, 6]), false);
    assert_eq!(can_reorder_doubled(vec![4, -2, 2, -4]), true);
}
