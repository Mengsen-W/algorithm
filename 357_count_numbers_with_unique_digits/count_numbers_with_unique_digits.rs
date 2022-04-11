/*
 * @Date: 2022-04-11 10:28:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-11 10:37:20
 * @FilePath: /algorithm/357_count_numbers_with_unique_digits/count_numbers_with_unique_digits.rs
 */
pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 10,
        _ => {
            let mut res = 10;
            let mut cur = 9;
            for i in 0..n - 1 {
                cur *= 9 - i;
                res += cur;
            }
            res
        }
    }
}

fn main() {
    assert_eq!(count_numbers_with_unique_digits(2), 91);
    assert_eq!(count_numbers_with_unique_digits(0), 1);
}
