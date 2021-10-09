/*
 * @Date: 2021-10-09 14:22:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-09 14:27:55
 */

use std::collections::BTreeMap;

struct SummaryRanges {
    data: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn add_num(&mut self, val: i32) {
        if let Some(_) = self.data.get(&val) {
            return;
        }
        let mut from = val;
        let mut to = val;
        match self.data.range(..val).next_back() {
            Some((&k, &v)) => {
                if v >= val {
                    return;
                } else if v == val - 1 {
                    from = k;
                }
            }
            None => {}
        };
        match self.data.range(val + 1..).next() {
            Some((&k, &v)) => {
                if k == val + 1 {
                    self.data.remove(&(val + 1));
                    to = v;
                }
            }
            None => {}
        };
        self.data.insert(from, to);
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for (&k, &v) in self.data.iter() {
            res.push(vec![k, v]);
        }
        res
    }
}

fn main() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]);
    summary_ranges.add_num(3);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
    summary_ranges.add_num(7);
    assert_eq!(
        summary_ranges.get_intervals(),
        vec![vec![1, 1], vec![3, 3], vec![7, 7]]
    );
    summary_ranges.add_num(2);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
    summary_ranges.add_num(6);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    summary_ranges.get_intervals();
}
