/*
 * @Date: 2022-10-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-14
 * @FilePath: /algorithm/940_distinct_subseq_ii/distinct_subseq_ii.rs
 */

pub fn distinct_subseq_ii(s: String) -> i32 {
    let m = 1000_000_007;
    let mut g = vec![0; 26];
    let mut x = 0;
    let b = 'a' as u8;
    for c in s.chars() {
        let a = c as u8 - b;
        let r = g[a as usize];
        g[a as usize] = (x % m + 1) % m;
        x = (x % m + 1 + x % m - r % m) % m;
        if x < 0 {
            x += m;
        }
    }
    x
}

fn main() {
    assert_eq!(distinct_subseq_ii(String::from("abc")), 7);
    assert_eq!(distinct_subseq_ii(String::from("aba")), 6);
    assert_eq!(distinct_subseq_ii(String::from("aaa")), 3);
}
