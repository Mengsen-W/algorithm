/*
 * @Date: 2021-09-14 08:52:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-14 10:45:15
 */

struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        fn can_form_by_deleting(word: &Vec<char>, s: &Vec<char>) -> bool {
            let (mut word_i, mut s_i) = (0, 0);
            let (word_size, s_size) = (word.len(), s.len());
            while word_i < word_size && s_i < s_size {
                if word[word_i] == s[s_i] {
                    word_i += 1;
                }
                s_i += 1;
            }
            word_i == word_size
        }
        let s: Vec<char> = s.chars().collect();
        let mut res: Vec<char> = Vec::new();
        let dictionary: Vec<Vec<char>> = dictionary.iter().map(|s| s.chars().collect()).collect();
        for word in dictionary {
            if can_form_by_deleting(&word, &s) {
                if word.len() > res.len() || word.len() == res.len() && word < res {
                    res = word;
                }
            }
        }
        res.iter().collect()
    }
}

fn main() {
    {
        let s = "abpcplea".to_string();
        let dictionary = vec![
            "ale".to_string(),
            "apple".to_string(),
            "monkey".to_string(),
            "plea".to_string(),
        ];
        let ans = "apple".to_string();
        assert_eq!(Solution::find_longest_word(s, dictionary), ans);
    }
    {
        let s = "abpcplea".to_string();
        let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let ans = "a".to_string();
        assert_eq!(Solution::find_longest_word(s, dictionary), ans);
    }
}
