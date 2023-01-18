/*
 * @Date: 2022-07-11
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-11
 * @FilePath: /algorithm/676_magic_dictionary/magic_dictionary.rs
 */

struct MagicDictionary {
    vec: Vec<String>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.vec.extend(dictionary);
    }

    fn search(&self, search_word: String) -> bool {
        let len = search_word.len();
        let words = search_word.bytes();
        for i in self.vec.iter().filter(|x| x.len() == len) {
            let mut num = 0;
            for (a, b) in i.bytes().zip(words.clone()) {
                if a != b {
                    num += 1;
                }
            }
            if num == 1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut m = MagicDictionary::new();
    m.build_dict(vec![String::from("hello"), String::from("leetcode")]);
    assert_eq!(m.search(String::from("hello")), false);
    assert_eq!(m.search(String::from("hhllo")), true);
    assert_eq!(m.search(String::from("hell")), false);
    assert_eq!(m.search(String::from("leetcoded")), false);
}
