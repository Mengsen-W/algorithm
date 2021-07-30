/*
 * @Date: 2021-07-30 09:49:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-30 10:22:38
 */

pub fn title_to_number(s: String) -> i32 {
    let mut res = 0;
    s.as_bytes()
        .iter()
        .for_each(|&x| res = res * 26 + (x - 'A' as u8 + 1) as i32);
    res.into()
}

fn main() {
    assert_eq!(title_to_number("A".to_string()), 1);
    assert_eq!(title_to_number("AB".to_string()), 28);
    assert_eq!(title_to_number("ZY".to_string()), 701);
}
