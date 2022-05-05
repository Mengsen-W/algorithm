/*
 * @Date: 2022-05-06 07:13:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-06 07:14:55
 * @FilePath: /algorithm/933_recent_counter/recent_counter.rs
 */

use std::collections::VecDeque;
struct RecentCounter {
    deque: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            deque: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.deque.push_back(t);
        while let Some(n) = self.deque.front() {
            if *n < (t - 3000) {
                self.deque.pop_front();
            } else {
                break;
            }
        }
        self.deque.len() as i32
    }
}

fn main() {
    let mut rc = RecentCounter::new();

    assert_eq!(rc.ping(1), 1);
    assert_eq!(rc.ping(100), 2);
    assert_eq!(rc.ping(3001), 3);
    assert_eq!(rc.ping(3002), 3);
}
