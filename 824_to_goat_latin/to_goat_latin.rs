/*
 * @Date: 2022-04-21 09:52:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-21 10:00:27
 * @FilePath: /algorithm/824_to_goat_latin/to_goat_latin.rs
 */

pub fn to_goat_latin(s: String) -> String {
    use std::collections::HashSet;
    let mut sr = String::new();
    let sv: Vec<&str> = s.split(' ').collect();
    let chs: HashSet<_> = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
        .iter()
        .cloned()
        .collect();
    let mut i = 2;
    for w in sv {
        let c = w.as_bytes()[0] as char;
        if !chs.contains(&c) {
            let mut t = String::from(w);
            t.remove(0);
            t.push(c);
            sr.push_str(&t);
        } else {
            sr.push_str(w);
        }
        sr.push('m');
        for _j in 0..i {
            sr.push('a');
        }
        sr.push(' ');
        i += 1;
    }
    sr.trim().to_string()
}

fn main() {
    assert_eq!(
        to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
    );
    assert_eq!(
        to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string()
    );
}
