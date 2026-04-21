/*
 * @Date: 2022-06-29
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-29
 * @FilePath: /algorithm/535_tiny_url/tiny_url.rs
 */

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
struct Codec {
    cache: HashMap<String, String>,
}

impl Codec {
    fn new() -> Self {
        Codec {
            cache: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let key = (since_the_epoch.as_secs() as i64 * 1000i64
            + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64)
            .to_string();
        self.cache
            .insert(format!("http://tinyurl.com/{}", key), longURL);
        "http://tinyurl.com/".to_string() + &key
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.cache.get(&shortURL).unwrap().to_string()
    }
}

fn main() {
    let url = String::from("https://leetcode.com/problems/design-tinyurl");
    let mut obj = Codec::new();
    let tiny = obj.encode(url.clone());
    let ans = obj.decode(tiny.clone());
    assert_eq!(url, ans);
}
