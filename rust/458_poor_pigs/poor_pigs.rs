/*
 * @Date: 2021-11-25 02:40:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-25 02:53:26
 */
pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    ((buckets as f64).ln() / ((minutes_to_test / minutes_to_die + 1) as f64).ln()).ceil() as i32
}

fn main() {
    assert_eq!(poor_pigs(1000, 15, 60), 5);
    assert_eq!(poor_pigs(4, 15, 15), 2);
    assert_eq!(poor_pigs(4, 15, 30), 2);
}
