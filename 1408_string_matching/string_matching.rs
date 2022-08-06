/*
 * @Date: 2022-08-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-06
 * @FilePath: /algorithm/1408_string_matching/string_matching.rs
 */

pub fn string_matching(words: Vec<String>) -> Vec<String> {
    words
        .clone()
        .into_iter()
        .filter(|word| words.join(" ").matches(word).nth(1).is_some())
        .collect::<Vec<String>>()
}

fn main() {
    {
        let words = vec!["mass", "as", "hero", "superhero"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ans = vec!["as", "hero"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(string_matching(words), ans)
    }

    {
        let words = vec!["leetcode", "et", "code"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ans = vec!["et", "code"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(string_matching(words), ans)
    }

    {
        let words = vec!["blue", "green", "bu"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ans: Vec<String> = Vec::new();
        assert_eq!(string_matching(words), ans)
    }
}
