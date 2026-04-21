/*
 * @Date: 2023-12-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-16
 * @FilePath: /algorithm/rust/2276_CountIntervals/CountIntervals.rs
 */

use std::cmp::{max, min};
use std::collections::BTreeMap;

pub struct CountIntervals {
    cnt: i32,
    mp: BTreeMap<i32, i32>,
}

impl CountIntervals {
    pub fn new() -> Self {
        CountIntervals {
            cnt: 0,
            mp: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, mut left: i32, mut right: i32) {
        let mut interval = self.mp.range(..=right).next_back();
        while let Some((l, r)) = interval {
            let l = *l;
            let r = *r;
            if l <= right && r >= left {
                left = min(left, l);
                right = max(right, r);
                self.cnt -= r - l + 1;
                self.mp.remove(&l);
            } else {
                break;
            }
            interval = self.mp.range(..=right).next_back();
        }
        self.cnt += right - left + 1;
        self.mp.insert(left, right);
    }

    pub fn count(&self) -> i32 {
        self.cnt
    }
}

fn main() {
    let mut c = CountIntervals::new();
    c.add(2, 3);
    c.add(7, 10);
    assert_eq!(c.count(), 6);
    c.add(5, 8);
    assert_eq!(c.count(), 8);
}
