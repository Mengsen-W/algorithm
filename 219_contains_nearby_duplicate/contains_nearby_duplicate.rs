/*
 * @Date: 2022-01-19 01:47:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-19 01:51:57
 */

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashSet;
    let mut cache = HashSet::new();

    for i in 0..nums.len() {
        if i as i32 - k > 0 {
            cache.remove(&nums[i - k as usize - 1]);
        }
        if !cache.insert(nums[i]) {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}
