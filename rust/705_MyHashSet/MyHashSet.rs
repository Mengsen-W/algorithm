/*
 * @Date: 2024-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-14
 * @FilePath: /algorithm/rust/705_MyHashSet/MyHashSet.rs
 */

use std::cell::RefCell;
use std::rc::Rc;

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Rc<RefCell<ListNode>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

struct MyHashSet {
    nodes: Vec<Option<Rc<RefCell<ListNode>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            nodes: vec![None; 10009],
        }
    }

    fn add(&mut self, key: i32) {
        let i = Self::hash(key);
        let (mut curr, next, mut last) = (self.nodes[i].clone(), ListNode::new(key), None);
        while let Some(node) = curr {
            if node.borrow().val == key {
                return;
            }
            last = Some(node.clone());
            curr = node.borrow().next.clone();
        }

        if let Some(last) = last {
            last.borrow_mut().next = Some(next);
        } else {
            self.nodes[i] = Some(next);
        }
    }

    fn remove(&mut self, key: i32) {
        let i = Self::hash(key);
        let (mut curr, mut prev): (_, Option<Rc<RefCell<ListNode>>>) =
            (self.nodes[i].clone(), None);
        while let Some(node) = curr {
            if node.borrow().val == key {
                if let Some(p) = prev {
                    p.borrow_mut().next = node.borrow_mut().next.take();
                } else {
                    self.nodes[i] = node.borrow_mut().next.take();
                }
                return;
            }
            prev = Some(node.clone());
            curr = node.borrow().next.clone();
        }
    }

    fn contains(&self, key: i32) -> bool {
        let i = Self::hash(key);
        let mut curr = self.nodes[i].clone();
        while let Some(node) = curr {
            if node.borrow().val == key {
                return true;
            }
            curr = node.borrow().next.clone();
        }
        false
    }

    fn hash(key: i32) -> usize {
        key as usize % 10008usize
    }
}

fn main() {
    let mut my_hash_set = MyHashSet::new();
    my_hash_set.add(1);
    my_hash_set.add(2);
    assert!(my_hash_set.contains(1));
    assert!(!my_hash_set.contains(3));
    my_hash_set.add(2);
    assert!(my_hash_set.contains(2));
    my_hash_set.remove(2);
    assert!(!my_hash_set.contains(2));
}
