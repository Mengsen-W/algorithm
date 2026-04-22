/*
 * @Date: 2022-06-09 09:47:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-09 12:59:43
 * @FilePath: /algorithm/497_random_points_in_non-overlapping_rectangles/random_points_in_non-overlapping_rectangles.rs
 */

use rand::prelude::*;
struct Solution {
    rects: Vec<Vec<i32>>,
    acc: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut acc = vec![0];
        for rect in rects.iter() {
            let area = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            acc.push(acc[acc.len() - 1] + area);
        }
        Self {
            rects,
            acc,
            rng: thread_rng(),
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let mut rnd = self.rng.gen_range(0, self.acc[self.acc.len() - 1]);
        let idx = self.acc.partition_point(|&x| x <= rnd) - 1;
        rnd -= self.acc[idx];
        let (a, b, c) = (self.rects[idx][0], self.rects[idx][1], self.rects[idx][3]);
        let x = rnd / (c - b + 1) + a;
        let y = rnd % (c - b + 1) + b;
        vec![x, y]
    }
}

fn main() {
    let solution = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    solution.pick();
}
