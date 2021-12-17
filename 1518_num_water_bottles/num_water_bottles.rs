/*
 * @Date: 2021-12-17 08:27:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-17 09:01:01
 */

pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let cnt = num_bottles / (num_exchange - 1);
    if num_bottles % (num_exchange - 1) == 0 {
        num_bottles + cnt - 1
    } else {
        num_bottles + cnt
    }
}

fn main() {
    assert_eq!(num_water_bottles(9, 3), 13);
    assert_eq!(num_water_bottles(15, 4), 19);
    assert_eq!(num_water_bottles(5, 5), 6);
    assert_eq!(num_water_bottles(2, 3), 2);
}
