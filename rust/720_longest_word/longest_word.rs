/*
 * @Date: 2022-03-17 14:29:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-17 14:41:21
 * @FilePath: /algorithm/720_longest_word/longest_word.rs
 * @Description: file content
 */

#[derive(Default)]
struct Trie {
    end: bool,
    next: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            cur = cur.next[i].get_or_insert_with(|| Default::default());
        }
        cur.end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut cur = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            match &cur.next[i] {
                Some(n) if n.end => cur = n,
                _ => return false,
            }
        }
        true
    }
}

struct Solution;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        for word in words.iter() {
            trie.insert(word);
        }
        let mut ans = "";
        for word in words.iter() {
            if !trie.search(word) || word.len() < ans.len() {
                continue;
            }
            if word.len() > ans.len() {
                ans = word;
            } else {
                ans = ans.min(word);
            }
        }
        ans.into()
    }
}

fn main() {
    assert_eq!(
        Solution::longest_word(
            vec!["w", "wo", "wor", "worl", "world"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "world".to_string()
    );
    assert_eq!(
        Solution::longest_word(
            vec!["a", "banana", "app", "appl", "ap", "apply", "apple"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "apple".to_string()
    );
}
