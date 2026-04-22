/*
 * @Date: 2024-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-09
 * @FilePath: /algorithm/rust/2386_k_sum/k_sum.rs
 */

struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sm = 0;
        let mut sms = vec![];
        for v in nums.iter() {
            if *v >= 0 {
                sm += *v as i64;
                sms.push(*v as i64);
            } else {
                sms.push(-*v as i64);
            }
        }
        sms.sort();
        let mut x = k;
        let mut queue = BinaryHeap::new();
        queue.push((sm, 0));
        while x > 1 {
            x -= 1;
            if let Some((s, i)) = queue.pop() {
                if i < sms.len() {
                    queue.push((s - sms[i], i + 1));
                    if i > 0 {
                        queue.push((s - sms[i] + sms[i - 1], i + 1));
                    }
                }
            }
        }
        queue.peek().unwrap().0
    }
}

fn main() {
    let tests = vec![(vec![2, 4, -2], 5, 2), (vec![1, -2, 3, 4, -10, 12], 16, 10)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::k_sum(nums, k), ans);
    }
}
