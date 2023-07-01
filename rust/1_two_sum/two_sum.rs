/*
 * @Date: 2021-08-26 11:45:01
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-01
 * @FilePath: /algorithm/rust/1_two_sum/two_sum.rs
 * @Description: file content
 */

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            if let Some(value) = map.get(&(target - nums[i])) {
                if *value != i as i32 {
                    return vec![*value as i32, i as i32];
                }
            }
            map.insert(nums[i], i as i32);
        }

        panic!("没有找到");
    }
}

fn main() {
    let test_map = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];

    for iter in test_map {
        assert_eq!(Solution::two_sum(iter.0, iter.1), iter.2);
    }
}
