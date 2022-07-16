/*
 * @Date: 2022-07-16
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-16
 * @FilePath: /algorithm/041_moving_average/moving_average.rs
 */

use std::collections::VecDeque;

struct MovingAverage {
    size: usize,
    queue: VecDeque<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            size: size as usize,
            queue: VecDeque::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() == self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(val);
        let sum = self.queue.iter().sum::<i32>() as f64;
        sum / (self.queue.len() as f64)
    }
}

fn main() {
    let mut m = MovingAverage::new(3);
    assert_eq!(m.next(1), 1.0);
    assert_eq!(m.next(10), 5.5);
    assert_eq!(m.next(3), 4.666666666666667);
    assert_eq!(m.next(5), 6.0);
}
