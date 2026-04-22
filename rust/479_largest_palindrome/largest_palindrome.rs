/*
 * @Date: 2022-04-16 16:10:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-16 16:55:01
 * @FilePath: /algorithm/479_largest_palindrome/largest_palindrome.rs
 */

pub fn largest_palindrome(n: i32) -> i32 {
    if n == 1 {
        return 9;
    }
    let upper = (10 as i32).pow(n as u32) - 1;
    for left in (upper / 10..upper).rev() {
        let (mut p, mut x) = (left as i64, left as i64);

        while x > 0 {
            p = p * 10 + x % 10;
            x /= 10;
        }
        x = upper as i64;
        while x * x >= p {
            if p % x == 0 {
                return (p % 1337) as i32;
            }
            x -= 1
        }
    }
    -1
}

fn main() {
    assert_eq!(largest_palindrome(2), 987);
    assert_eq!(largest_palindrome(1), 9);
}
