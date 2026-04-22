/*
 * @Date: 2024-03-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-04
 * @FilePath: /algorithm/rust/232_MyQueue/MyQueue.rs
 */

struct MyQueue {
    ins: Vec<i32>,
    out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            ins: vec![],
            out: vec![],
        }
    }

    fn tansfer(&mut self) {
        while let Some(x) = self.ins.pop() {
            self.out.push(x);
        }
    }

    fn push(&mut self, x: i32) {
        self.ins.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.out.is_empty() {
            self.tansfer();
        }

        self.out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out.is_empty() {
            self.tansfer();
        }

        *self.out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.out.is_empty() && self.ins.is_empty()
    }
}

fn main() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1);
    my_queue.push(2);
    assert_eq!(my_queue.peek(), 1);
    assert_eq!(my_queue.pop(), 1);
    assert_eq!(my_queue.empty(), false);
}
