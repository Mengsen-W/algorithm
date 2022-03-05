/*
 * @Date: 2022-03-05 00:33:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:45:08
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.rs
 */

pub fn find_lu_slength(a: String, b: String) -> i32 {
    match a == b {
        true => -1,
        false => a.len().max(b.len()) as i32,
    }
}

fn main() {
    assert_eq!(find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
    assert_eq!(find_lu_slength("aaa".to_string(), "bbb".to_string()), 3);
    assert_eq!(find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
}
