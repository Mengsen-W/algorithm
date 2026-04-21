/*
 * @Date: 2024-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-03
 * @FilePath: /algorithm/rust/225_MyStack/MyStack.rs
 */

use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    out1: bool,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
            out1: false,
        }
    }

    fn push(&mut self, x: i32) {
        let (q1, q2) = if !self.out1 {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };
        q1.push_back(x);
        while let Some(val) = q2.pop_front() {
            q1.push_back(val);
        }
        self.out1 = !self.out1;
    }

    fn pop(&mut self) -> i32 {
        if self.out1 {
            self.q1.pop_front().unwrap()
        } else {
            self.q2.pop_front().unwrap()
        }
    }

    fn top(&mut self) -> i32 {
        if self.out1 {
            *self.q1.front().unwrap()
        } else {
            *self.q2.front().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

fn main() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}
