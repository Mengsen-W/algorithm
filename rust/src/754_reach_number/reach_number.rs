/*
 * @Date: 2022-11-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-04
 * @FilePath: /algorithm/754_reach_number/reach_number.rs
 */

pub fn reach_number(target: i32) -> i32 {
    let mut target = target.abs();
    let mut k = 0;
    while target > 0 {
        k += 1;
        target -= k;
    }

    if target % 2 == 0 {
        k
    } else {
        k + 1 + k % 2
    }
}

fn main() {
    assert_eq!(reach_number(2), 3);
    assert_eq!(reach_number(3), 2);
}
