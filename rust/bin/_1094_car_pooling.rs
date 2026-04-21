/*
 * @Date: 2023-12-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-02
 * @FilePath: /algorithm/rust/1094_car_pooling/car_pooling.rs
 */

struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut d = BTreeMap::new();
        for t in &trips {
            let num = t[0];
            let from = t[1];
            let to = t[2];
            *d.entry(from).or_insert(0) += num;
            *d.entry(to).or_insert(0) -= num;
        }
        let mut s = 0;
        for (_, v) in &d {
            s += v;
            if s > capacity {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 1, 5], vec![3, 3, 7]], 4, false),
        (vec![vec![2, 1, 5], vec![3, 3, 7]], 5, true),
    ];

    for (trips, capacity, ans) in tests {
        assert_eq!(Solution::car_pooling(trips, capacity), ans);
    }
}
