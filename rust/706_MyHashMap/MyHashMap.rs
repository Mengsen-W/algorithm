/*
 * @Date: 2024-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-15
 * @FilePath: /algorithm/rust/706_MyHashMap/MyHashMap.rs
 */

use std::collections::LinkedList;

const BASE: i32 = 769;
struct MyHashMap {
    list: Vec<LinkedList<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            list: vec![LinkedList::new(); 769],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        for item in self.list[(key % BASE) as usize].iter_mut() {
            if item.0 == key {
                item.1 = value;
                return;
            }
        }
        self.list[(key % BASE) as usize].push_back((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        for item in self.list[(key % BASE) as usize].iter() {
            if item.0 == key {
                return item.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let mut idx = None;
        for (i, item) in self.list[(key % BASE) as usize].iter().enumerate() {
            if item.0 == key {
                idx = Some(i);
                break;
            }
        }
        if let Some(idx) = idx {
            let mut split_list = self.list[(key % BASE) as usize].split_off(idx);
            split_list.pop_front();
            self.list[(key % BASE) as usize].append(&mut split_list);
        }
    }
}

fn main() {
    let mut map = MyHashMap::new();
    map.put(1, 1);
    map.put(2, 2);
    assert_eq!(map.get(1), 1);
    assert_eq!(map.get(3), -1);
    map.put(2, 1);
    assert_eq!(map.get(2), 1);
    map.remove(2);
    assert_eq!(map.get(2), -1);
}
