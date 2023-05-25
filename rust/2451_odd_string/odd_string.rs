/*
 * @Date: 2023-05-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-25
 * @FilePath: /algorithm/rust/2451_odd_string/odd_string.rs
 */

pub fn odd_string(words: Vec<String>) -> String {
    let mut mp = std::collections::HashMap::new();

    for word in words.iter() {
        let mut tmp = String::new();
        let bytes = word.as_bytes();
        let n = bytes.len();

        for i in 1..n {
            tmp.push_str(&(bytes[i] as i32 - bytes[i - 1] as i32).to_string());
            tmp.push(',');
        }

        mp.entry(tmp).or_insert(vec![]).push(word);
    }

    for (_, v) in mp {
        if v.len() == 1 {
            return v[0].to_string();
        }
    }

    "".to_string()
}

fn main() {
    {
        let words = vec!["adc", "wzy", "abc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = "abc".to_string();
        assert_eq!(odd_string(words), ans);
    }

    {
        let words = vec!["aaa", "bob", "ccc", "ddd"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = "bob".to_string();
        assert_eq!(odd_string(words), ans);
    }
}
