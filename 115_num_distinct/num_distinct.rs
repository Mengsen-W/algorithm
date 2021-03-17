/*
 * @Date: 2021-03-17 08:26:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-17 09:27:36
 */

fn num_distinct(s: String, t: String) -> i32 {
    let mut f = if t.len() <= s.len() {
        vec![0; t.len() + 1]
    } else {
        return 0;
    };
    f[0] = 1;

    s.chars().for_each(|sc| {
        t.chars()
            .rev()
            .enumerate()
            .for_each(|(i, tc)| f[t.len() - i] += if sc == tc { f[t.len() - i - 1] } else { 0 });
    });

    f[t.len()]
}

fn main() {
    assert_eq!(
        num_distinct(String::from("rabbbit"), String::from("rabbit")),
        3
    );
    assert_eq!(
        num_distinct(String::from("babgbag"), String::from("bag")),
        5
    );
}
