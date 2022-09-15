/*
 * @Date: 2022-09-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-15
 * @FilePath: /algorithm/672_flip_lights/flip_lights.rs
 */

pub fn flip_lights(n: i32, presses: i32) -> i32 {
    if presses > 2 && n > 2 {
        return 8;
    }
    if n < 3 {
        1 + (presses > 0) as i32 * n + (presses > 1 && n > 1) as i32
    } else {
        1 + 3 * presses
    }
}

fn main() {
    assert_eq!(flip_lights(1, 1), 2);
    assert_eq!(flip_lights(2, 1), 3);
    assert_eq!(flip_lights(3, 1), 4);
}
