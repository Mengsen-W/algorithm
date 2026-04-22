/*
 * @Date: 2022-01-16 02:48:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-16 03:02:19
 */

struct Solution {
    size: usize,
    nums: Vec<i32>,
}

impl Solution {
    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut nums = vec![];
        let mut size = 0;
        while let Some(mut node) = head {
            nums.push(node.val);
            size += 1;
            head = node.next.take();
        }
        Self { size, nums }
    }

    fn get_random(&self) -> i32 {
        let mut rand = 0;
        unsafe {
            use std::arch::x86_64::_rdrand32_step;
            _rdrand32_step(&mut rand);
        }
        self.nums[rand as usize % self.size]
    }
}

use rand::prelude::*;
struct Solution {
    rng: ThreadRng,
    head: Option<Box<ListNode>>,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self {
            head,
            rng: thread_rng(),
        }
    }

    fn get_random(&mut self) -> i32 {
        let mut i = 1;
        let mut ans = 0;
        let mut node = self.head.as_ref();
        while node.is_some() {
            if self.rng.gen_range(0, i) == 0 {
                ans = node.unwrap().val;
            }
            i += 1;
            node = node.unwrap().next.as_ref();
        }
        ans
    }
}

fn main() {}