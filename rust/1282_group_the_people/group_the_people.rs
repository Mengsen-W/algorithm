/*
 * @Date: 2022-08-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-12
 * @FilePath: /algorithm/1282_group_the_people/group_the_people.rs
 */

pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    let mut r = vec![];
    let mut map = HashMap::new();

    for (i, x) in group_sizes.iter().enumerate() {
        let e = map.entry(x).or_insert(vec![]);
        e.push(i as i32);
        if e.len() == *x as usize {
            r.push(e.clone());
            map.remove(x);
        }
    }
    r
}

fn main() {
    assert_eq!(
        group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
    );
    assert_eq!(
        group_the_people(vec![2, 1, 3, 3, 3, 2]),
        vec![vec![2, 3, 4], vec![1], vec![0, 5]]
    );
}
