/*
 * @Date: 2022-01-23 14:49:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-23 15:05:24
 */

use std::collections::{BTreeMap, HashMap, HashSet};

struct StockPrice {
    time_tab: HashMap<i32, i32>,
    cur_time: i32,
    cur_price: i32,
    price_tab: BTreeMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            time_tab: HashMap::with_capacity(10000),
            cur_time: 0,
            cur_price: 0,
            price_tab: BTreeMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if timestamp >= self.cur_time {
            self.cur_time = timestamp;
            self.cur_price = price;
        }
        if let Some(p) = self.time_tab.get_mut(&timestamp) {
            let old_price = *p;
            *p = price;
            let t = self.price_tab.get_mut(&old_price).unwrap();
            t.remove(&timestamp);
            if t.is_empty() {
                self.price_tab.remove(&old_price);
            }
            let t = self.price_tab.entry(price).or_insert(HashSet::new());
            t.insert(timestamp);
        } else {
            self.time_tab.insert(timestamp, price);
            let t = self.price_tab.entry(price).or_insert(HashSet::new());
            t.insert(timestamp);
        }
    }

    fn current(&self) -> i32 {
        self.cur_price
    }

    fn maximum(&self) -> i32 {
        *self.price_tab.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_tab.iter().next().unwrap().0
    }
}

fn main() {
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10);
    stock_price.update(2, 5);
    assert_eq!(stock_price.current(), 5);
    assert_eq!(stock_price.maximum(), 10);
    stock_price.update(1, 3);
    assert_eq!(stock_price.maximum(), 5);
    stock_price.update(4, 2);
    assert_eq!(stock_price.minimum(), 2);
}
