/*
 * @Date: 2021-07-12 08:28:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:53:18
 */

use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data
            .entry(key)
            .or_insert(Vec::new())
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.data.get(&key) {
            match v.binary_search_by(|probe| probe.0.cmp(&timestamp)) {
                Ok(i) => v[i].1.clone(),
                Err(i) => {
                    if i == 0 {
                        String::new()
                    } else {
                        v[i - 1].1.clone()
                    }
                }
            }
        } else {
            String::new()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {
    {
        let mut tv = TimeMap::new();
        tv.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(tv.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(tv.get("foo".to_string(), 3), "bar".to_string());
        tv.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(tv.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(tv.get("foo".to_string(), 5), "bar2".to_string());
    }
    {
        let mut tv = TimeMap::new();
        tv.set("love".to_string(), "high".to_string(), 10);
        tv.set("love".to_string(), "low".to_string(), 20);
        assert_eq!(tv.get("love".to_string(), 5), String::new());
        assert_eq!(tv.get("love".to_string(), 10), "high".to_string());
        assert_eq!(tv.get("love".to_string(), 15), "high".to_string());
        assert_eq!(tv.get("love".to_string(), 20), "low".to_string());
    }
}
