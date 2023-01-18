/*
 * @Date: 2021-11-22 02:14:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-22 02:21:11
 */

use rand::Rng;

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut ans = self.nums.clone();
        let n = ans.len();
        for i in 0..n {
            let j = i + rand::thread_rng().gen_range(0, n - i);
            ans.swap(i, j);
        }
        ans
    }
}

fn main() {
    let mut s = Solution::new(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("{:?}", s.shuffle());
    println!("{:?}", s.reset());
    println!("{:?}", s.shuffle());
}
