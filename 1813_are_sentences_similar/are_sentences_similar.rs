/*
 * @Date: 2023-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-16
 * @FilePath: /algorithm/1813_are_sentences_similar/are_sentences_similar.rs
 */

pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let mut words1 = sentence1.split(' ');
    let mut words2 = sentence2.split(' ');
    let mut ans = 0;
    while let (Some(x), Some(y)) = (words1.next(), words2.next()) {
        if x == y {
            ans += 1
        } else {
            break;
        }
    }
    let mut words1 = sentence1.split(' ');
    let mut words2 = sentence2.split(' ');
    while let (Some(x), Some(y)) = (words1.next_back(), words2.next_back()) {
        if x == y {
            ans += 1
        } else {
            break;
        }
    }
    sentence1
        .split(' ')
        .count()
        .min(sentence2.split(' ').count()) as i32
        <= ans
}

fn main() {
    {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();
        assert!(are_sentences_similar(sentence1, sentence2));
    }

    {
        let sentence1 = "of".to_string();
        let sentence2 = "A lot of words".to_string();
        assert!(!are_sentences_similar(sentence1, sentence2));
    }

    {
        let sentence1 = "Eating right now".to_string();
        let sentence2 = "Eating".to_string();
        assert!(are_sentences_similar(sentence1, sentence2));
    }

    {
        let sentence1 = "Luky".to_string();
        let sentence2 = "Lucccky".to_string();
        assert!(!are_sentences_similar(sentence1, sentence2));
    }
}
