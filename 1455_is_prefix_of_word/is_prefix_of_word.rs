/*
 * @Date: 2022-08-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-21
 * @FilePath: /algorithm/1455_is_prefix_of_word/is_prefix_of_word.rs
 */

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    for (i, word) in sentence.split_whitespace().enumerate() {
        if word.starts_with(&search_word) {
            return i as i32 + 1;
        }
    }
    -1
}

fn main() {
    {
        let sentence = String::from("i love eating burger");
        let search_word = String::from("burg");
        let ans = 4;
        assert_eq!(is_prefix_of_word(sentence, search_word), ans);
    }
    {
        let sentence = String::from("this problem is an easy problem");
        let search_word = String::from("pro");
        let ans = 2;
        assert_eq!(is_prefix_of_word(sentence, search_word), ans);
    }
    {
        let sentence = String::from("i am tired");
        let search_word = String::from("you");
        let ans = -1;
        assert_eq!(is_prefix_of_word(sentence, search_word), ans);
    }
}
