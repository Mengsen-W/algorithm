/*
 * @Date: 2023-10-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-08
 * @FilePath: /algorithm/rust/2034_StockPrice/StockPrice.rs
 */

use std::collections::BTreeMap;
use std::collections::HashMap;
struct StockPrice {
    cur: i32,
    time_price: HashMap<i32, i32>,
    price_cnt: BTreeMap<i32, i32>,
}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            cur: 0,
            time_price: HashMap::new(),
            price_cnt: BTreeMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.cur = self.cur.max(timestamp);
        if let Some(old) = self.time_price.insert(timestamp, price) {
            *self.price_cnt.entry(old).or_insert(0) -= 1;
            if self.price_cnt[&old] <= 0 {
                self.price_cnt.remove(&old);
            }
        }
        *self.price_cnt.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        self.time_price[&self.cur]
    }

    fn maximum(&self) -> i32 {
        *self.price_cnt.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_cnt.iter().next().unwrap().0
    }
}

fn main() {
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10); // 时间戳为 [1] ，对应的股票价格为 [10] 。
    stock_price.update(2, 5); // 时间戳为 [1,2] ，对应的股票价格为 [10,5] 。
    assert_eq!(stock_price.current(), 5); // 返回 5 ，最新时间戳为 2 ，对应价格为 5 。
    assert_eq!(stock_price.maximum(), 10); // 返回 10 ，最高价格的时间戳为 1 ，价格为 10 。
    stock_price.update(1, 3); // 之前时间戳为 1 的价格错误，价格更新为 3 。
                              // 时间戳为 [1,2] ，对应股票价格为 [3,5] 。
    assert_eq!(stock_price.maximum(), 5); // 返回 5 ，更正后最高价格为 5 。
    stock_price.update(4, 2); // 时间戳为 [1,2,4] ，对应价格为 [3,5,2] 。
    assert_eq!(stock_price.minimum(), 2); // 返回 2 ，最低价格时间戳为 4 ，价格为 2 。
}
