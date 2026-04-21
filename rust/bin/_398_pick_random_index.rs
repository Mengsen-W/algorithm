/*
 * @Date: 2022-04-25 09:28:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-25 09:37:45
 * @FilePath: /algorithm/398_pick_random_index/pick_random_index.rs
 */

use std::time::{SystemTime, UNIX_EPOCH};

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        let (mut index, mut count) = (-1, 0);
        for i in 0..self.nums.len() {
            if self.nums[i] == target {
                count += 1;
                let rand = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_micros();
                if rand % count == 0 {
                    index = i as i32;
                }
            }
        }
        index
    }
}

fn main() {
    let s = Solution::new(vec![1, 2, 3, 3, 3]);
    assert_eq!(s.pick(1), 0);
    assert_eq!(s.pick(2), 1);
    assert_eq!(s.pick(3) == 2 || s.pick(3) == 3 || s.pick(3) == 4, true);
}
