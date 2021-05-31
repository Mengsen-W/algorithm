/*
 * @Date: 2021-05-31 09:05:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-31 09:12:40
 */

fn is_power_of_four(n: i32) -> bool {
    n > 0 && n & (n - 1) == 0 && n % 3 == 1
}

fn main() {
    assert!(is_power_of_four(16));
    assert!(!is_power_of_four(5));
    assert!(is_power_of_four(1));
}
