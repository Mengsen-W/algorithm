/*
 * @Date: 2022-08-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-02
 * @FilePath: /algorithm/622_MyCircularQueue/MyCircularQueue.rs
 */

struct MyCircularQueue {
    arr: Vec<i32>,
    curr_size: usize,
    rear: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            arr: vec![0; k as usize],
            curr_size: 0,
            rear: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        return if self.is_full() {
            false
        } else {
            self.arr[self.rear] = value;
            self.rear = (self.rear + 1) % self.arr.len();
            self.curr_size += 1;
            true
        };
    }

    fn de_queue(&mut self) -> bool {
        return if self.is_empty() {
            false
        } else {
            self.curr_size -= 1;
            true
        };
    }

    fn front(&self) -> i32 {
        return if self.is_empty() {
            -1
        } else {
            self.arr[(self.rear + self.arr.len() - self.curr_size) % self.arr.len()]
        };
    }

    fn rear(&self) -> i32 {
        return if self.is_empty() {
            -1
        } else {
            self.arr[(self.rear + self.arr.len() - 1) % self.arr.len()]
        };
    }

    fn is_empty(&self) -> bool {
        self.curr_size == 0
    }

    fn is_full(&self) -> bool {
        self.curr_size == self.arr.len()
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(3);
    assert_eq!(obj.en_queue(1), true);
    assert_eq!(obj.en_queue(2), true);
    assert_eq!(obj.en_queue(3), true);
    assert_eq!(obj.en_queue(4), false);
    assert_eq!(obj.rear(), 3);
    assert_eq!(obj.is_full(), true);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.en_queue(4), true);
    assert_eq!(obj.rear(), 4);
    assert_eq!(obj.front(), 2);
}
