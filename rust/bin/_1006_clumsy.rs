/*
 * @Date: 2021-04-04 18:55:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:57:23
 */

fn clumsy(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 7,
        _ if n % 4 == 0 => n + 1,
        _ if n % 4 <= 2 => n + 2,
        _ => n - 1,
    }
}
fn main() {
    assert_eq!(clumsy(4), 7);
    assert_eq!(clumsy(10), 12);
}
