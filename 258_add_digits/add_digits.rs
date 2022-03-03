/*
 * @Date: 2022-03-03 00:27:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-03 00:40:49
 * @FilePath: /algorithm/258_add_digits/add_digits.rs
 */

pub fn add_digits1(mut num: i32) -> i32 {
    while num >= 10 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        num = sum;
    }
    num
}

pub fn add_digits2(num: i32) -> i32 {
    (num - 1) % 9 + 1
}

fn main() {
    assert_eq!(add_digits1(38), 2);
    assert_eq!(add_digits2(38), 2);

    assert_eq!(add_digits1(0), 0);
    assert_eq!(add_digits2(0), 0);
}
