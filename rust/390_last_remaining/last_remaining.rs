/*
 * @Date: 2022-01-02 01:34:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-02 01:41:55
 */

pub fn last_remaining(n: i32) -> i32 {
    let (mut a1, mut an) = (1, n);
    let (mut k, mut cnt, mut step) = (0, n, 1);
    while cnt > 1 {
        if k % 2 == 0 {
            a1 = a1 + step;
            an = if cnt % 2 == 0 { an - step } else { an };
        } else {
            a1 = if cnt % 2 == 0 { a1 } else { a1 + step };
            an = an - step;
        }
        k += 1;
        cnt = cnt >> 1;
        step = step << 1;
    }
    a1
}

fn main() {
    assert_eq!(last_remaining(9), 6);
    assert_eq!(last_remaining(1), 1);
}
