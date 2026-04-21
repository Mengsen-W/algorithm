/*
 * @Date: 2024-05-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-11
 * @FilePath: /algorithm/rust/2391_garbage_collection/garbage_collection.rs
 */

struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut distance: HashMap<char, i32> = HashMap::new();
        let mut res = 0;
        let mut cur_dis = 0;
        for (i, item) in garbage.iter().enumerate() {
            res += item.len() as i32;
            if i > 0 {
                cur_dis += travel[i - 1];
            }
            for c in item.chars() {
                *distance.entry(c).or_insert(0) = cur_dis;
            }
        }
        for v in distance.values() {
            res += v;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["G", "P", "GP", "GG"], vec![2, 4, 3], 21),
        (vec!["MMM", "PGM", "GP"], vec![3, 10], 37),
    ];

    for (garbage, travel, ans) in tests {
        assert_eq!(
            Solution::garbage_collection(garbage.iter().map(|s| s.to_string()).collect(), travel),
            ans
        );
    }
}
