struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct NumberContainers {
    nums: HashMap<i32, i32>,
    heaps: HashMap<i32, BinaryHeap<Reverse<i32>>>,
}

impl NumberContainers {
    pub fn new() -> Self {
        Self {
            nums: HashMap::new(),
            heaps: HashMap::new(),
        }
    }

    pub fn change(&mut self, index: i32, number: i32) {
        self.nums.insert(index, number);
        self.heaps
            .entry(number)
            .or_insert(BinaryHeap::new())
            .push(Reverse(index));
    }

    pub fn find(&mut self, number: i32) -> i32 {
        if let Some(heap) = self.heaps.get_mut(&number) {
            while let Some(&Reverse(top)) = heap.peek() {
                if self.nums.get(&top) != Some(&number) {
                    heap.pop();
                } else {
                    return top;
                }
            }
        }
        -1
    }
}

fn main() {
    let mut nc = NumberContainers::new();
    assert_eq!(nc.find(10), -1);
    nc.change(2, 10);
    nc.change(1, 10);
    nc.change(3, 10);
    nc.change(5, 10);
    assert_eq!(nc.find(10), 1);
    nc.change(1, 20);
    assert_eq!(nc.find(10), 2);
}
