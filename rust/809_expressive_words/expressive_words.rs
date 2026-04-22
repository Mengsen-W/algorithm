/*
 * @Date: 2022-11-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-25
 * @FilePath: /algorithm/809_expressive_words/expressive_words.rs
 */

pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
    fn expand(s: &String, t: &String) -> bool {
        let (s, t): (Vec<char>, Vec<char>) = (s.chars().collect(), t.chars().collect());
        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < t.len() {
            if s[i] != t[j] {
                return false;
            }
            let ch = s[i];
            let mut cnti = 0;
            while i < s.len() && s[i] == ch {
                cnti += 1;
                i += 1;
            }
            let mut cntj = 0;
            while j < t.len() && t[j] == ch {
                cntj += 1;
                j += 1;
            }
            if cnti < cntj {
                return false;
            }
            if cnti != cntj && cnti < 3 {
                return false;
            }
        }
        i == s.len() && j == t.len()
    }

    let mut ans = 0;
    for word in words {
        if expand(&s, &word) {
            ans += 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(
        expressive_words(
            String::from("heeellooo"),
            vec!["hello", "hi", "helo"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        1
    );
}
