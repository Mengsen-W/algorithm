/*
 * @Date: 2021-05-30 09:22:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-30 09:36:44
 */

fn is_power_of_two_1(n: i32) -> bool {
    n > 0 && n & (n - 1) == 0
}
fn is_power_of_two_2(n: i32) -> bool {
    n > 0 && (n & -n) == n
}
fn is_power_of_two_3(n: i32) -> bool {
    const BIG: i32 = 1 << 30;
    n > 0 && BIG % n == 0
}

fn main() {
    assert!(is_power_of_two_1(1));
    assert!(is_power_of_two_2(1));
    assert!(is_power_of_two_3(1));
    assert!(is_power_of_two_1(16));
    assert!(is_power_of_two_2(16));
    assert!(is_power_of_two_3(16));
    assert!(!is_power_of_two_1(3));
    assert!(!is_power_of_two_2(3));
    assert!(!is_power_of_two_3(3));
    assert!(is_power_of_two_1(4));
    assert!(is_power_of_two_2(4));
    assert!(is_power_of_two_3(4));
    assert!(!is_power_of_two_1(5));
    assert!(!is_power_of_two_2(5));
    assert!(!is_power_of_two_3(5));
}
