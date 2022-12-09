/*
 * @Date: 2022-12-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-09
 * @FilePath: /algorithm/1780_check_powers_of_three/check_powers_of_three.rs
 */

pub fn check_powers_of_three(n: i32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    return true;
}

fn main() {
    assert!(check_powers_of_three(12));
    assert!(check_powers_of_three(91));
    assert!(!check_powers_of_three(21));
}
