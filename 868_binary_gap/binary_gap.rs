/*
 * @Date: 2022-04-24 09:47:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-24 09:56:12
 * @FilePath: /algorithm/868_binary_gap/binary_gap.rs
 */

pub fn binary_gap(n: i32) -> i32 {
    let (mut idx, mut pre, mut ret, mut n) = (0, -1, 0, n);
    while n > 0 {
        if n & 1 == 1 {
            if pre != -1 {
                ret = ret.max(idx - pre);
            }
            pre = idx;
        }
        idx += 1;
        n >>= 1;
    }
    ret
}

fn main() {
    assert_eq!(binary_gap(22), 2);
    assert_eq!(binary_gap(8), 0);
    assert_eq!(binary_gap(5), 2);
}
