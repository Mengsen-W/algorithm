/*
 * @Date: 2022-06-03 23:11:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-03 23:17:27
 * @FilePath: /algorithm/829_consecutive_numbers_sum/consecutive_numbers_sum.rs
 */

pub fn consecutive_numbers_sum(mut n: i32) -> i32 {
    let (mut cnt, k) = (0, ((2 * n) as f64).sqrt() as i32);
    for i in 1..=k {
        n -= i;
        cnt += if n % i == 0 { 1 } else { 0 }
    }
    cnt
}

fn main() {
    assert_eq!(consecutive_numbers_sum(5), 2);
    assert_eq!(consecutive_numbers_sum(9), 3);
    assert_eq!(consecutive_numbers_sum(15), 4);
}
