/*
 * @Date: 2022-07-14
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-14
 * @FilePath: /algorithm/745_word_filter/word_filter.rs
 */

struct WordFilter {
    map: std::collections::HashMap<String, i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut w = WordFilter {
            map: std::collections::HashMap::new(),
        };
        for i in 0..words.len() {
            let word = &words[i];
            let m = word.len();
            for prefix in 1..=m {
                for suffix in 1..=m {
                    let key = format!("{}{}{}", &word[0..prefix], "#", &word[m - suffix..m]);
                    w.map.insert(key, i as i32);
                }
            }
        }
        w
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let target: String = format!("{}{}{}", pref, "#", suff);
        if self.map.contains_key(&target) {
            *self.map.get(&target).unwrap()
        } else {
            -1
        }
    }
}

fn main() {
    let w = WordFilter::new(vec![String::from("apple")]);
    assert_eq!(w.f(String::from("a"), String::from("e")), 0);
}
