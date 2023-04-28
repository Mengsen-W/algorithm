/*
 * @Date: 2023-04-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-28
 * @FilePath: /algorithm/rust/1172_DinnerPlates/DinnerPlates.rs
 */

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct DinnerPlates {
    map: HashMap<i32, Option<Vec<i32>>>,
    ext: BinaryHeap<i32>,
    nful: BinaryHeap<Reverse<i32>>,
    capacity: i32,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            ext: BinaryHeap::new(),
            nful: BinaryHeap::new(),
            capacity: capacity,
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(&top_idx) = self.nful.peek() {
            let entb = self.map.get_mut(&top_idx.0).unwrap();
            if let Some(ent) = entb {
                if (ent.len() as i32) < self.capacity {
                    ent.push(val);
                    if ent.len() as i32 == self.capacity {
                        self.nful.pop();
                    }
                    return;
                } else {
                    self.nful.pop();
                }
            } else {
                *entb = Some(vec![val]);
                self.ext.push(top_idx.0);
                return;
            }
        }

        let idx = self.map.len() as i32;
        self.map.insert(idx, Some(vec![val]));
        self.ext.push(idx);
        self.nful.push(Reverse(idx));
    }

    fn pop(&mut self) -> i32 {
        while let Some(&top) = self.ext.peek() {
            let v = self.map.get_mut(&top).unwrap();
            match v {
                Some(u) if !u.is_empty() => {
                    let a = u.pop().unwrap();
                    if u.is_empty() {
                        *v = None;
                        self.ext.pop();
                    }
                    self.nful.push(Reverse(top));
                    return a;
                }
                Some(_) => {
                    *v = None;
                    self.ext.pop();
                }
                None => {
                    self.ext.pop();
                }
            }
        }
        -1
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let en = self.map.get_mut(&index);
        if en.is_none() {
            return -1;
        }
        let en = en.unwrap();
        match en {
            Some(v) => {
                if !v.is_empty() {
                    let a = v.pop().unwrap();
                    if v.is_empty() {
                        *en = None;
                    }
                    self.nful.push(Reverse(index));
                    a
                } else {
                    -1
                }
            }
            None => -1,
        }
    }
}

fn main() {
    let mut d = DinnerPlates::new(2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(5);
    assert_eq!(d.pop_at_stack(0), 2);
    d.push(20);
    d.push(21);
    assert_eq!(d.pop_at_stack(0), 20);
    assert_eq!(d.pop_at_stack(2), 21);
    assert_eq!(d.pop(), 5);
    assert_eq!(d.pop(), 4);
    assert_eq!(d.pop(), 3);
    assert_eq!(d.pop(), 1);
    assert_eq!(d.pop(), -1);
}
