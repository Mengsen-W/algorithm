/*
 * @Date: 2022-02-08 23:56:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-09 00:42:17
 */

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    nums.iter()
        .fold(std::collections::HashMap::new(), |mut cnt, &num| {
            res += cnt.get(&(num + k)).unwrap_or(&0) + cnt.get(&(num - k)).unwrap_or(&0);
            *cnt.entry(num).or_insert(0) += 1;
            cnt
        });
    res
}

fn main() {
    assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    assert_eq!(count_k_difference(vec![1, 3], 3), 0);
    assert_eq!(count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
}
