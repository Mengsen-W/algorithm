/*
 * @Date: 2022-06-21 09:45:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:49:31
 * @FilePath: /algorithm/715_range_module/range_module.rs
 * @Description: file content
 */

use std::cmp;
use std::collections::BTreeMap;

struct RangeModule {
    ranges: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);
        let rl: Vec<_> = self
            .ranges
            .range(left..)
            .take_while(|(_, &l)| l <= right)
            .map(|(&r, &l)| (r, l))
            .collect();
        for (r, l) in rl {
            left = cmp::min(left, l);
            right = cmp::max(right, r);
            self.ranges.remove(&r);
        }

        self.ranges.insert(right, left);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.ranges.range(right..).any(|(_, &l)| l <= left)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let rl: Vec<_> = self
            .ranges
            .range(left..)
            .take_while(|(_, &l)| l < right)
            .map(|(&r, &l)| (r, l))
            .collect();
        for (r, l) in rl {
            self.ranges.remove(&r);
            if left > l {
                self.ranges.insert(left, l);
            }
            if right < r {
                self.ranges.insert(r, right);
            }
        }
    }
}

fn main() {
    let mut r = RangeModule::new();
    r.add_range(10, 20);
    r.remove_range(14, 16);
    assert_eq!(r.query_range(10, 14), true);
    assert_eq!(r.query_range(13, 15), false);
    assert_eq!(r.query_range(16, 17), true);
}
