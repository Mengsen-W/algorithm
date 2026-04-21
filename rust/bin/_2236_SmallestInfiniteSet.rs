/*
 * @Date: 2023-11-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-29
 * @FilePath: /algorithm/rust/2236_SmallestInfiniteSet/SmallestInfiniteSet.rs
 */

use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    set: BTreeSet<i32>,
    cnt: i32,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            cnt: 0,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(&n) = self.set.iter().next() {
            self.set.remove(&n);
            n
        } else {
            self.cnt += 1;
            self.cnt
        }
    }

    fn add_back(&mut self, num: i32) {
        if num <= self.cnt {
            self.set.insert(num);
        }
    }
}

fn main() {
    let mut s = SmallestInfiniteSet::new();
    s.add_back(2);
    assert_eq!(s.pop_smallest(), 1);
    assert_eq!(s.pop_smallest(), 2);
    assert_eq!(s.pop_smallest(), 3);
    s.add_back(1);
    assert_eq!(s.pop_smallest(), 1);
    assert_eq!(s.pop_smallest(), 4);
    assert_eq!(s.pop_smallest(), 5);
}
