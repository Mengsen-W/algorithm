/*
 * @Date: 2021-05-07 09:54:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-07 10:14:24
 */

fn xor_operation(n: i32, start: i32) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        ans ^= start + i * 2;
    }
    ans
}

fn main() {
    assert_eq!(xor_operation(5, 0), 8);
    assert_eq!(xor_operation(4, 3), 8);
    assert_eq!(xor_operation(1, 7), 7);
    assert_eq!(xor_operation(10, 5), 2);
}
