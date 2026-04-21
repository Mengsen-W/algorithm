/*
 * @Date: 2022-10-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-23
 * @FilePath: /algorithm/1768_merge_alternately/merge_alternately.rs
 */

pub fn merge_alternately(word1: String, word2: String) -> String {
    let (mut word1_iter, mut word2_iter, mut ret) = (
        word1.chars().peekable(),
        word2.chars().peekable(),
        "".to_string(),
    );
    while word1_iter.peek().is_some() || word2_iter.peek().is_some() {
        if let Some(ch) = word1_iter.next() {
            ret.push(ch)
        }
        if let Some(ch) = word2_iter.next() {
            ret.push(ch)
        }
    }
    ret
}

fn main() {
    {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        let ans = String::from("apbqcr");
        assert_eq!(merge_alternately(word1, word2), ans);
    }

    {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        let ans = String::from("apbqrs");
        assert_eq!(merge_alternately(word1, word2), ans);
    }

    {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        let ans = String::from("apbqcd");
        assert_eq!(merge_alternately(word1, word2), ans);
    }
}
