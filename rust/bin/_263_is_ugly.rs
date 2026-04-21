/*
 * @Date: 2021-04-10 08:55:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-10 09:10:16
 */

fn is_ugly(mut n: i32) -> bool {
    if n < 1 {
        return false;
    }
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

fn main() {
    assert!(is_ugly(6));
    assert!(is_ugly(8));
    assert!(!is_ugly(14));
    assert!(is_ugly(1));
}
