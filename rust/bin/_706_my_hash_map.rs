/*
 * @Date: 2021-03-14 09:07:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-14 09:13:24
 * @FilePath: \algorithm\706_my_hash_map\my_hash_map.rs
 */

struct MyHashMap {
    hash_: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashMap {
            hash_: vec![-1; 1000001],
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.hash_[key as usize] = value;
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&mut self, key: i32) -> i32 {
        self.hash_[key as usize]
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.hash_[key as usize] = -1;
    }
}

fn main() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    assert_eq!(obj.get(1), 1);
    obj.remove(1);
    assert_eq!(obj.get(1), -1);
}
