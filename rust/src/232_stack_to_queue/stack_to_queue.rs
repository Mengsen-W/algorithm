/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-05 10:10:46
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-05 10:42:54
 */

#[derive(Default)]
struct MyQueue {
    in_stk: Vec<i32>,
    out_stk: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.in_stk.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.out_stk.is_empty() {
            self.in_to_out();
        }
        self.out_stk.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.out_stk.is_empty() {
            self.in_to_out();
        }
        *self.out_stk.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.in_stk.is_empty() && self.out_stk.is_empty()
    }

    fn in_to_out(&mut self) {
        while !self.in_stk.is_empty() {
            self.out_stk.push(self.in_stk.pop().unwrap());
        }
    }
}

fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(1, obj.peek());
    assert_eq!(1, obj.pop());
    assert!(!obj.empty());
}
