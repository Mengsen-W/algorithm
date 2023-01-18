/*
 * @Date: 2022-08-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-15
 * @FilePath: /algorithm/641_MyCircularDeque/MyCircularDeque.rs
 */

use std::collections::VecDeque;
struct MyCircularDeque {
    cache: VecDeque<i32>,
    capacity: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            cache: VecDeque::with_capacity(k as usize),
            capacity: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.cache.push_front(value);
            true
        } else {
            false
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.cache.push_back(value);
            true
        } else {
            false
        }
    }

    fn delete_front(&mut self) -> bool {
        if !self.is_empty() {
            self.cache.pop_front();
            true
        } else {
            false
        }
    }

    fn delete_last(&mut self) -> bool {
        if !self.is_empty() {
            self.cache.pop_back();
            true
        } else {
            false
        }
    }

    fn get_front(&self) -> i32 {
        *self.cache.front().unwrap_or(&-1)
    }

    fn get_rear(&self) -> i32 {
        *self.cache.back().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn is_full(&self) -> bool {
        self.cache.len() == self.capacity
    }
}

fn main() {
    let mut m = MyCircularDeque::new(3);
    assert_eq!(m.insert_last(1), true);
    assert_eq!(m.insert_last(2), true);
    assert_eq!(m.insert_front(3), true);
    assert_eq!(m.insert_front(4), false);
    assert_eq!(m.get_rear(), 2);
    assert_eq!(m.is_full(), true);
    assert_eq!(m.delete_last(), true);
    assert_eq!(m.insert_front(4), true);
    assert_eq!(m.get_front(), 4);
}
