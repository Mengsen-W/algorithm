/*
 * @Date: 2022-08-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-08
 * @FilePath: /algorithm/761_make_largest_special/make_largest_special.rs
 */

pub fn make_largest_special(s: String) -> String {
    let mut cnt = 0;
    let mut v: Vec<_> = s
        .split_inclusive(|c| {
            cnt += if '1' == c { 1 } else { -1 };
            cnt == 0
        })
        .map(|t| format!("1{}0", make_largest_special(t[1..t.len() - 1].into())))
        .collect();

    v.sort_unstable_by(|a, b| b.cmp(a));
    v.concat()
}

fn main() {
    assert_eq!(make_largest_special("11011000".to_string()), "11100100");
}
