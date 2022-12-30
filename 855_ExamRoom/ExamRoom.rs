/*
 * @Date: 2022-12-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-30
 * @FilePath: /algorithm/855_ExamRoom/ExamRoom.rs
 */

use std::collections::{BTreeMap, BTreeSet};
struct ExamRoom {
    nodes: BTreeSet<i32>,
    area: BTreeMap<i32, BTreeMap<i32, i32>>,
    n: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            nodes: BTreeSet::new(),
            area: BTreeMap::from([(n, BTreeMap::from([(-1, n)]))]),
            n,
        }
    }

    fn seat(&mut self) -> i32 {
        let mut size = *self.area.iter().rev().next().unwrap().0;
        if let Some(x) = self.area.get_mut(&size) {
            let a = x.iter().next().unwrap();
            let mut ans = if *a.0 == -1 {
                0
            } else if *a.1 == self.n {
                self.n - 1
            } else {
                *a.0 + (*a.1 - *a.0) / 2
            };
            let mut sub = *a.0;
            // 偶数空位的区间x，要考虑x-1的区间
            if size & 1 == 0 && self.n != ans + 1 {
                if let Some(x) = self.area.get_mut(&(size - 1)) {
                    let a = x.iter().next().unwrap();
                    let second = if *a.0 == -1 {
                        0
                    } else if *a.1 == self.n {
                        self.n - 1
                    } else {
                        *a.0 + (*a.1 - *a.0) / 2
                    };
                    if second < ans {
                        size = size - 1;
                        sub = *a.0;
                        ans = second;
                    }
                }
            }

            let a = sub;
            if let Some(x) = self.area.get_mut(&size) {
                if let Some(b) = x.remove(&a) {
                    if x.is_empty() {
                        self.area.remove(&size);
                    }
                    self.nodes.insert(ans);
                    // println!("{:?}", self.area);
                    self.area
                        .entry(ans - a - 1)
                        .or_insert_with(|| BTreeMap::new())
                        .insert(a, ans);
                    self.area
                        .entry(b - ans - 1)
                        .or_insert_with(|| BTreeMap::new())
                        .insert(ans, b);
                    return ans;
                }
            }
        }
        0
    }

    fn leave(&mut self, p: i32) {
        let a = self.nodes.range(..p).rev().next().copied().unwrap_or(-1);
        self.nodes.remove(&p);
        let size = p - a - 1;
        if let Some(x) = self.area.get_mut(&size) {
            x.remove(&a);
            if x.is_empty() {
                self.area.remove(&size);
            }
        }
        let b = self.nodes.range(p + 1..).next().copied().unwrap_or(self.n);
        let size = b - p - 1;
        if let Some(x) = self.area.get_mut(&size) {
            x.remove(&p);
            if x.is_empty() {
                self.area.remove(&size);
            }
        }
        self.area
            .entry(b - a - 1)
            .or_insert_with(|| BTreeMap::new())
            .insert(a, b);
    }
}

fn main() {
    let mut e = ExamRoom::new(10);
    assert_eq!(e.seat(), 0);
    assert_eq!(e.seat(), 9);
    assert_eq!(e.seat(), 4);
    assert_eq!(e.seat(), 2);
    e.leave(4);
    assert_eq!(e.seat(), 5);
}
