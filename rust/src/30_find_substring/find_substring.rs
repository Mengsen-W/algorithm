/*
 * @Date: 2022-06-23
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-23
 * @FilePath: /algorithm/30_find_substring/find_substring.rs
 */

struct Solution;

impl Solution {
    // https://www.strchr.com/hash_functions
    fn hash_word(s: &[u8]) -> usize {
        let mut hash: usize = 0xB16B00B5;
        let m: usize = 33;
        for i in 0..s.len() {
            hash = m ^ hash * s[i] as usize;
        }
        hash
    }
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ret = vec![];
        let len = words.len();
        if s.len() == 0 || len == 0 || s.len() < words[0].len() * len {
            return ret;
        }
        let word_len = words[0].len();
        let mut words_hash: usize = 0;
        for word in words.into_iter() {
            words_hash += Self::hash_word(word.as_bytes());
        }
        let sb = s.as_bytes();
        for i in 0..(sb.len() - word_len * len + 1) {
            let mut tmp_hash: usize = 0;
            for j in 0..len {
                tmp_hash += Self::hash_word(&sb[(i + j * word_len)..(i + (j + 1) * word_len)]);
            }
            if tmp_hash == words_hash {
                ret.push(i as i32);
            }
        }
        ret
    }
}

fn main() {
    assert_eq!(
        Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")]
        ),
        vec![0, 9]
    );

    assert_eq!(
        Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec![
                String::from("word"),
                String::from("good"),
                String::from("best"),
                String::from("word")
            ]
        ),
        vec![]
    );

    assert_eq!(
        Solution::find_substring(
            String::from("barfoofoobarthefoobarman"),
            vec![
                String::from("bar"),
                String::from("foo"),
                String::from("the")
            ]
        ),
        vec![6, 9, 12]
    );
}
