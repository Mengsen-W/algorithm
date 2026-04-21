/*
 * @Date: 2022-03-05 00:33:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:45:08
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.rs
 */
struct Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        match a == b {
            true => -1,
            false => a.len().max(b.len()) as i32,
        }
    }
}

fn main() {
    let tests = vec![("aba", "cdc", 3), ("aaa", "bbb", 3), ("aaa", "aaa", -1)];

    for (a, b, ans) in tests {
        assert_eq!(Solution::find_lu_slength(a.to_string(), b.to_string()), ans);
    }
}
