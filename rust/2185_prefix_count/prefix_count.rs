/*
 * @Date: 2023-01-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-08
 * @FilePath: /algorithm/2185_prefix_count/prefix_count.rs
 */

pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words
        .iter()
        .filter(|w| w.len() >= pref.len() && w.as_str().starts_with(pref.as_str()))
        .count() as i32
}

fn main() {
    {
        let words = vec!["pay", "attention", "practice", "attend"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pref = String::from("at");
        let ans = 2;
        assert_eq!(prefix_count(words, pref), ans);
    }

    {
        let words = vec!["leetcode", "win", "loops", "success"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pref = String::from("code");
        let ans = 0;
        assert_eq!(prefix_count(words, pref), ans);
    }
}
