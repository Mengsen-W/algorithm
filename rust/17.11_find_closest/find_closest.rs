/*
 * @Date: 2022-05-28 10:18:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:27:28
 * @FilePath: /algorithm/17.11_find_closest/find_closest.rs
 */

pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut ans = words.len() as i32;
    let mut index1 = None;
    let mut index2 = None;
    for (i, word) in words.iter().enumerate() {
        if word == &word1 {
            index1 = Some(i);
        } else if word == &word2 {
            index2 = Some(i);
        } else if let (Some(index1), Some(index2)) = (index1, index2) {
            ans = ans.min((index1 as i32 - index2 as i32).abs())
        }
    }
    ans
}

fn main() {
    assert_eq!(
        find_closest(
            vec![
                "I",
                "am",
                "a",
                "student",
                "from",
                "a",
                "university",
                "in",
                "a",
                "city"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            "a".to_string(),
            "student".to_string()
        ),
        1
    );
}
