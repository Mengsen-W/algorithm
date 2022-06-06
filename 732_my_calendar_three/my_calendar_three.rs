/*
 * @Date: 2022-06-06 09:37:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-06 10:04:43
 * @FilePath: /algorithm/732_my_calendar_three/my_calendar_three.rs
 */

use std::collections::HashMap;

struct MyCalendarThree {
    tree: HashMap<i32, i32>,
    lazy: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            tree: HashMap::new(),
            lazy: HashMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.update(start, end - 1, 0, 1000000000, 1);
        *self.tree.get(&1).unwrap_or(&0)
    }

    fn update(&mut self, start: i32, end: i32, left: i32, right: i32, index: i32) {
        if right < start || end < left {
            return;
        }

        if start <= left && right <= end {
            *self.tree.entry(index).or_insert(0) += 1;
            *self.lazy.entry(index).or_insert(0) += 1;
        } else {
            let mid = left + (right - left) / 2;
            self.update(start, end, left, mid, 2 * index);
            self.update(start, end, mid + 1, right, 2 * index + 1);
            let &v1 = self.lazy.get(&index).unwrap_or(&0);
            let &v2 = self.tree.get(&(index * 2)).unwrap_or(&0);
            let &v3 = self.tree.get(&(index * 2 + 1)).unwrap_or(&0);
            self.tree.insert(index, v1 + v2.max(v3));
        }
    }
}

// Your MyCalendarThree object will be instantiated and called as such:
// let obj = MyCalendarThree::new();
// let ret_1: i32 = obj.book(start, end);

fn main() {
    let mut my_calendar_three = MyCalendarThree::new();
    assert_eq!(my_calendar_three.book(10, 20), 1);
    assert_eq!(my_calendar_three.book(50, 60), 1);
    assert_eq!(my_calendar_three.book(10, 40), 2);
    assert_eq!(my_calendar_three.book(5, 15), 3);
    assert_eq!(my_calendar_three.book(5, 10), 3);
    assert_eq!(my_calendar_three.book(25, 55), 3);
}
