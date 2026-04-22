/*
 * @Date: 2021-08-27 18:27:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-27 18:31:41
 */

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MedianFinder {
    low: (i32, i32),
    high: (i32, i32),
    data: BTreeMap<i32, i32>,
    len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MedianFinder {
            low: (std::i32::MIN, 0),
            high: (std::i32::MAX, 1),
            data: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn add_num(&mut self, num: i32) {
        match self.data.get_mut(&num) {
            Some(t) => {
                *t += 1;
            }
            None => {
                self.data.insert(num, 1);
            }
        }
        self.len += 1;
        if self.len == 1 {
            self.low = (num, 1);
            self.high = (num, 1);
            return;
        }
        let merge = self.len % 2 == 1;
        if num > self.high.0 {
            if merge {
                self.low = self.high;
            } else {
                if self.high.1 != *self.data.get(&self.high.0).unwrap() {
                    self.high.1 += 1;
                } else {
                    let (&a, &_) = self.data.range(self.high.0 + 1..).next().unwrap();
                    self.high = (a, 1);
                }
            }
        } else if num == self.high.0 {
            self.low = self.high;
            if !merge {
                self.high.1 += 1;
            }
        } else if num < self.low.0 {
            if merge {
                self.high = self.low;
            } else {
                if self.low.1 != 1 {
                    self.low.1 -= 1;
                } else {
                    let (&a, &b) = self.data.range(..self.low.0).next_back().unwrap();
                    self.low = (a, b);
                }
            }
        } else if num == self.low.0 {
            self.low.1 += 1;
            self.high = self.low;
            // 不合并的和 num == self.low.0 一样 ，不会进入这个分支
        } else {
            if merge {
                self.low = (num, 1);
                self.high = (num, 1);
            } else {
                if self.low.0 > num {
                    self.low = (num, 1);
                } else {
                    self.high = (num, 1);
                }
            }
        }
        // println!("{:?}", self);
    }

    pub fn find_median(&self) -> f64 {
        (self.low.0 + self.high.0) as f64 / 2.0
    }
}

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    assert_eq!(obj.find_median(), 2.0);
}
