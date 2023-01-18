/*
 * @Date: 2022-03-31 13:11:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 13:36:07
 * @FilePath: /algorithm/728_self_dividing_numbers/self_dividing_numbers.rs
 */

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .filter(|&n| {
            let mut x = n;
            while x > 0 {
                let digit = x % 10;
                if digit == 0 || n % digit != 0 {
                    return false;
                }
                x /= 10;
            }
            true
        })
        .collect()
}

fn main() {
    assert_eq!(
        self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
    assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
}
