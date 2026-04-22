/*
 * @Date: 2022-05-25 09:54:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-25 10:08:21
 * @FilePath: /algorithm/467_find_substring_in_wrapround_string/find_substring_in_wrapround_string.rs
 */

pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    let mut dp = vec![0; 26];
    let mut k = 0;
    let pp = p.as_bytes();

    for i in 0..p.len() {
        if i > 0 && (pp[i] + 26 - pp[i - 1]) % 26 == 1 {
            k += 1;
        } else {
            k = 1;
        }

        dp[(pp[i] - b'a') as usize] = dp[(pp[i] - b'a') as usize].max(k);
    }

    dp.iter().sum()
}

fn main() {
    assert_eq!(find_substring_in_wrapround_string(String::from("a")), 1);
    assert_eq!(find_substring_in_wrapround_string(String::from("cac")), 2);
    assert_eq!(find_substring_in_wrapround_string(String::from("zab")), 6);
}
