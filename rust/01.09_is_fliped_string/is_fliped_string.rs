/*
 * @Date: 2022-09-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-29
 * @FilePath: /algorithm/01.09_is_fliped_string/is_fliped_string.rs
 */

pub fn is_fliped_string(s1: String, s2: String) -> bool {
    s1.len() == s2.len() && s2.repeat(2).contains(&s1)
}

fn main() {
    assert!(is_fliped_string(
        String::from("waterbottle"),
        String::from("erbottlewat")
    ));
    assert!(!is_fliped_string(String::from("aa"), String::from("aba")));
}
