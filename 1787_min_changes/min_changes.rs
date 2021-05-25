/*
 * @Date: 2021-05-25 09:34:25
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-25 20:09:23
 */

fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let number_freq: HashMap<i32, i32> = HashMap::new();
    let dp_pre = vec![i32::MAX; 1024];
    let dp = vec![i32::MAX; 1024];

    for i in 0..k {
        let mut size = 0.len();
        dp_pre = dp;
        let min_dp_pre = dp_pre.iter().fold(std::i32::MIN, |a, b| a.min(*b));
        number_freq.clear();
        for j in i..nums.size() {
            let counter = number_freq.entry(j).or_insert(0);
            *counter += 1;
            size += 1;
        }
        for j in 0..1024 {
            dp[j] = min_dp_pre;
        }
        for 
    }
}

fn main() {
    assert_eq!(min_changes(vec![1, 2, 0, 3, 0], 1), 3);
    // assert_eq!(min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
    assert_eq!(min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
}
