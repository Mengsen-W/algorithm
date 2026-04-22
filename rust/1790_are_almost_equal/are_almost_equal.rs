/*
 * @Date: 2022-10-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-11
 * @FilePath: /algorithm/1790_are_almost_equal/are_almost_equal.rs
 */

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
    let (mut i, mut j) = (-1, -1);
    for idx in 0..s1.len() {
        if s1[idx] != s2[idx] {
            if i < 0 {
                i = idx as i32;
            } else if j < 0 {
                j = idx as i32;
            } else {
                return false;
            }
        }
    }
    i < 0 || j >= 0 && s1[i as usize] == s2[j as usize] && s1[j as usize] == s2[i as usize]
}

fn main() {
    assert!(are_almost_equal(String::from("bank"), String::from("kanb")));
    assert!(!are_almost_equal(
        String::from("attack"),
        String::from("defend")
    ));
    assert!(are_almost_equal(String::from("kelb"), String::from("kelb")));
    assert!(!are_almost_equal(
        String::from("abcd"),
        String::from("dcba")
    ));
}
