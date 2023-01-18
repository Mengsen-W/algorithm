/*
 * @Date: 2022-09-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-27
 * @FilePath: /algorithm/01.02_check_permutation/check_permutation.rs
 */

pub fn check_permutation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        false
    } else {
        let mut cnt: [i32; 26] = [0; 26];
        for (&a, &b) in s1.as_bytes().iter().zip(s2.as_bytes().iter()) {
            cnt[(a - b'a') as usize] += 1;
            cnt[(b - b'a') as usize] -= 1
        }
        cnt.iter().filter(|&&x| x != 0).count() == 0
    }
}

fn main() {
    assert!(check_permutation(String::from("abc"), String::from("bca")));
    assert!(!check_permutation(String::from("abc"), String::from("bad")));
}
