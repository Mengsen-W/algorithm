/*
 * @Date: 2021-11-14 02:00:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-14 02:12:16
 */

struct MapSum {
    cnt: std::collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {
            cnt: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.cnt.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut sum = 0;
        for (key, val) in &self.cnt {
            if key.starts_with(&prefix) {
                sum += val;
            }
        }
        sum
    }
}

fn main() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    assert_eq!(obj.sum("ap".to_string()), 3);
    obj.insert("app".to_string(), 2);
    assert_eq!(obj.sum("ap".to_string()), 5);
}
