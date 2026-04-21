/*
 * @Date: 2021-03-13 08:32:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-13 08:37:24
 * @FilePath: \algorithm\705_my_hash_set\my_hash_set.rs
 */

struct MyHashSet {
    set_: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            set_: vec![false; 1000000 + 1],
        }
    }

    fn add(&mut self, key: i32) {
        self.set_[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.set_[key as usize] = false;
    }

    /** Returns true if this set contains the specified element */
    fn contains(&mut self, key: i32) -> bool {
        self.set_[key as usize]
    }
}

fn main() {
    let mut obj: MyHashSet = MyHashSet::new();
    obj.add(1);
    assert!(obj.contains(1));
    obj.remove(1);
    assert!(!obj.contains(1));
    assert!(!obj.contains(2));
}
