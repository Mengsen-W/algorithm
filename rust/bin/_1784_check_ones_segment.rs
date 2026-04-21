/*
 * @Date: 2022-10-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-03
 * @FilePath: /algorithm/1784_check_ones_segment/check_ones_segment.rs
 */

pub fn check_ones_segment(s: String) -> bool {
    !s.contains("01")
}

fn main() {
    assert!(!check_ones_segment(String::from("1001")));
    assert!(check_ones_segment(String::from("110")));
}
