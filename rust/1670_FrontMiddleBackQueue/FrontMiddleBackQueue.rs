/*
 * @Date: 2023-11-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-28
 * @FilePath: /algorithm/rust/1670_FrontMiddleBackQueue/FrontMiddleBackQueue.rs
 */

use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }

    // 调整长度，保证 0 <= right.len() - left.len() <= 1
    // 从而保证可以在正中间插入删除元素
    fn balance(&mut self) {
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap());
        } else if self.right.len() > self.left.len() + 1 {
            self.left.push_back(self.right.pop_front().unwrap());
        }
    }

    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.balance();
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() < self.right.len() {
            self.left.push_back(val);
        } else {
            self.right.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.balance();
    }

    fn pop_front(&mut self) -> i32 {
        if self.right.is_empty() {
            // 整个队列为空
            return -1;
        }
        let val = if self.left.is_empty() {
            self.right.pop_front().unwrap()
        } else {
            self.left.pop_front().unwrap()
        };
        self.balance();
        val
    }

    fn pop_middle(&mut self) -> i32 {
        if self.right.is_empty() {
            // 整个队列为空
            return -1;
        }
        if self.left.len() == self.right.len() {
            return self.left.pop_back().unwrap();
        }
        self.right.pop_front().unwrap()
    }

    fn pop_back(&mut self) -> i32 {
        if self.right.is_empty() {
            // 整个队列为空
            return -1;
        }
        let val = self.right.pop_back().unwrap();
        self.balance();
        val
    }
}

fn main() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1); // [1]
    q.push_back(2); // [1, 2]
    q.push_middle(3); // [1, 3, 2]
    q.push_middle(4); // [1, 4, 3, 2]
    assert_eq!(q.pop_front(), 1); // 返回 1 -> [4, 3, 2]
    assert_eq!(q.pop_middle(), 3); // 返回 3 -> [4, 2]
    assert_eq!(q.pop_middle(), 4); // 返回 4 -> [2]
    assert_eq!(q.pop_back(), 2); // 返回 2 -> []
    assert_eq!(q.pop_front(), -1); // 返回 -1 -> [] （队列为空）
}
