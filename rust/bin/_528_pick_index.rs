/*
 * @Date: 2021-08-30 13:12:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-30 13:16:01
 */

use rand::{thread_rng, Rng};

struct Solution {
    prefix_sum: Vec<i32>,
}

impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }

        Self { prefix_sum: w }
    }

    fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1, self.prefix_sum.last().unwrap() + 1);

        match self.prefix_sum.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

fn main() {
    {
        let s = Solution::new(vec![1]);
        assert_eq!(s.pick_index(), 0);
    }
    {
        let s = Solution::new(vec![1, 3]);
        assert_eq!(s.pick_index(), 1);
    }
}
