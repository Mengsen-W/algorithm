/*
 * @Date: 2024-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-18
 * @FilePath: /algorithm/rust/2007_find_original_array/find_original_array.rs
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        changed.sort_unstable();
        let mut count = HashMap::new();
        for num in &changed {
            *count.entry(*num).or_insert(0) += 1;
        }
        let mut res = Vec::new();
        for a in changed {
            if *count.get(&a).unwrap_or(&0) == 0 {
                continue;
            }
            *count.get_mut(&a).unwrap() -= 1;

            if *count.get(&(a * 2)).unwrap_or(&0) == 0 {
                return vec![];
            }
            *count.get_mut(&(a * 2)).unwrap() -= 1;
            res.push(a);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 4, 2, 6, 8], vec![1, 3, 4]),
        (vec![6, 3, 0, 1], vec![]),
        (vec![1], vec![]),
    ];

    for (changed, ans) in tests {
        assert_eq!(Solution::find_original_array(changed), ans);
    }
}
