/*
 * @Date: 2021-05-27 09:41:11
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-27 10:51:46
 */

fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

fn hamming_distance_move_bits(x: i32, y: i32) -> i32 {
    let mut s = x ^ y;
    let mut ret = 0;
    while s != 0 {
        ret += s & 1;
        s >>= 1;
    }
    ret
}

fn hamming_distance_bk(x: i32, y: i32) -> i32 {
    let mut s = x ^ y;
    let mut ret = 0;
    while s != 0 {
        s &= s - 1;
        ret += 1;
    }
    ret
}

fn main() {
    assert_eq!(hamming_distance(1, 4), 2);
    assert_eq!(hamming_distance_move_bits(1, 4), 2);
    assert_eq!(hamming_distance_bk(1, 4), 2);
}
