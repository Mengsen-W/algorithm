/*
 * @Date: 2022-01-05 01:15:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-05 01:35:23
 */

pub fn modify_string(s: String) -> String {
    let mut s_arr: Vec<char> = s.chars().collect();
    s.chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '?')
        .for_each(|(i, _)| {
            (0..3).for_each(|j| {
                let c = char::from(j as u8 + 'a' as u8);
                if (i <= 0 || s_arr[i - 1] != c) && (i >= s.len() - 1 || s_arr[i + 1] != c) {
                    s_arr[i] = c;
                }
            });
        });
    return s_arr.into_iter().collect();
}

fn main() {
    assert_eq!(modify_string("?zs".to_string()), "czs".to_string());
    assert_eq!(modify_string("ubv?w".to_string()), "ubvcw".to_string());
    assert_eq!(modify_string("j?qg??b".to_string()), "jcqgcab".to_string());
    assert_eq!(
        modify_string("??yw?ipkj?".to_string()),
        "cbywcipkjc".to_string()
    );
}
