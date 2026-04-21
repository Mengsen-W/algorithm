/*
 * @Date: 2022-01-26 01:44:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-26 02:45:14
 */

use std::collections::HashMap;

struct DetectSquares {
    cnt: HashMap<i32, HashMap<i32, i32>>,
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares {
            cnt: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        *(self
            .cnt
            .entry(point[1])
            .or_insert(HashMap::new())
            .entry(point[0])
            .or_insert(0)) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (x, y) = (&point[0], &point[1]);
        if !self.cnt.contains_key(y) {
            return 0;
        }

        let y_cnt = &self.cnt.get(y).unwrap();
        for (col, col_cnt) in self.cnt.iter() {
            if col != y {
                let d = &(col - y);
                res += (if col_cnt.contains_key(x) {
                    col_cnt.get(x).unwrap()
                } else {
                    &0
                }) * (if y_cnt.contains_key(&(x + d)) {
                    y_cnt.get(&(x + d)).unwrap()
                } else {
                    &0
                }) * (if col_cnt.contains_key(&(x + d)) {
                    col_cnt.get(&(x + d)).unwrap()
                } else {
                    &0
                });

                res += (if col_cnt.contains_key(x) {
                    col_cnt.get(x).unwrap()
                } else {
                    &0
                }) * (if y_cnt.contains_key(&(x - d)) {
                    y_cnt.get(&(x - d)).unwrap()
                } else {
                    &0
                }) * (if col_cnt.contains_key(&(x - d)) {
                    col_cnt.get(&(x - d)).unwrap()
                } else {
                    &0
                });
            }
        }
        res
    }
}

fn main() {
    let mut ds = DetectSquares::new();
    ds.add(vec![3, 10]);
    ds.add(vec![11, 2]);
    ds.add(vec![3, 2]);
    assert_eq!(ds.count(vec![11, 10]), 1);
    assert_eq!(ds.count(vec![14, 8]), 0);
    ds.add(vec![11, 2]);
    assert_eq!(ds.count(vec![11, 10]), 2);
}
