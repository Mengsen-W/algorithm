/*
 * @Date: 2022-12-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-08
 * @FilePath: /algorithm/1812_square_is_white/square_is_white.rs
 */

pub fn square_is_white(coordinates: String) -> bool {
    let v = coordinates.as_bytes();
    v[0] % 2 != v[1] % 2
}

fn main() {
    assert!(!square_is_white(String::from("a1")));
    assert!(square_is_white(String::from("h3")));
    assert!(!square_is_white(String::from("c7")));
}
