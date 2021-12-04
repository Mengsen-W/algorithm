/*
 * @Date: 2021-12-04 05:49:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:49:22
 */

use rand::{rngs::ThreadRng, thread_rng, Rng};

use std::collections::HashMap;
struct Solution {
    m: i32,
    n: i32,
    total: i32,
    rng: ThreadRng,
    map: HashMap<i32, i32>,
}
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            total: m * n,
            rng: thread_rng(),
            map: HashMap::new(),
        }
    }
    fn flip(&mut self) -> Vec<i32> {
        let x = self.rng.gen_range(0, self.total);
        self.total -= 1;
        let pos = *self.map.get(&x).unwrap_or(&x);
        self.map
            .insert(x, *self.map.get(&self.total).unwrap_or(&self.total));
        vec![pos / self.n, pos % self.n]
    }
    fn reset(&mut self) {
        self.total = self.m * self.n;
        self.map.clear();
    }
}

fn main() {}
