/*
 * @Date: 2022-07-07
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-07
 * @FilePath: /algorithm/648_replace_words/replace_words.rs
 */

struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for s in dictionary {
            trie.insert(s.as_str());
        }
        sentence
            .split_whitespace()
            .map(|s| trie.map(s))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Clone)]
struct Trie {
    next: Vec<Option<Box<Trie>>>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            next: vec![None; 26],
            end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut ptr = self;
        for c in word.bytes() {
            let index = (c - b'a') as usize;
            if ptr.next[index].is_none() {
                ptr.next[index] = Some(Box::new(Trie::new()));
            }
            ptr = ptr.next[index].as_mut().unwrap();
        }
        ptr.end = true;
    }

    fn map<'a>(&self, word: &'a str) -> &'a str {
        let mut ptr = self;
        for (i, c) in word.char_indices() {
            if ptr.end {
                return &word[0..i];
            }
            let index = (c as u8 - b'a') as usize;
            if ptr.next[index].is_some() {
                ptr = ptr.next[index].as_ref().unwrap();
            } else {
                break;
            }
        }
        word
    }
}

fn main() {
    {
        let dictionary = vec![
            String::from("cat"),
            String::from("bat"),
            String::from("rat"),
        ];
        let sentence = String::from("the cattle was rattled by the battery");
        let ans = String::from("the cat was rat by the bat");
        assert_eq!(Solution::replace_words(dictionary, sentence), ans);
    }

    {
        let dictionary = vec![String::from("a"), String::from("b"), String::from("c")];
        let sentence = String::from("aadsfasf absbs bbab cadsfafs");
        let ans = String::from("a a b c");
        assert_eq!(Solution::replace_words(dictionary, sentence), ans);
    }
}
