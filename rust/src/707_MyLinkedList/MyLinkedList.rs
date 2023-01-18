/*
 * @Date: 2022-09-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-23
 * @FilePath: /algorithm/707_MyLinkedList/MyLinkedList.rs
 */

use core::ptr::NonNull;

struct MyLinkedList {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
}

struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
    element: i32,
}

impl Node {
    fn new(element: i32) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }

    fn into_element(self: Box<Self>) -> i32 {
        self.element
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.len as i32 || index < 0 {
            return -1;
        }

        let mut tmp = self.head;
        unsafe {
            for i in 0..index {
                tmp = (*tmp.unwrap().as_ptr()).next;
            }
            return (*tmp.unwrap().as_ptr()).element;
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let mut node = Box::new(Node::new(val));
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    fn pop_at_head(&mut self) {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
        });
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut node = Box::new(Node::new(val));
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    fn pop_at_tail(&mut self) {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
        });
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut node = Box::new(Node::new(val));
        let node = Some(Box::leak(node).into());

        if index <= self.len as i32 {
            if index <= 0 {
                self.add_at_head(val);
            } else if index == self.len as i32 {
                self.add_at_tail(val);
            } else {
                let mut tmp = self.head;

                unsafe {
                    for i in 0..index - 1 {
                        tmp = (*tmp.unwrap().as_ptr()).next;
                    }

                    let mut nxt = (*tmp.unwrap().as_ptr()).next.take();
                    (*tmp.unwrap().as_ptr()).next = node;
                    (*node.unwrap().as_ptr()).prev = tmp;

                    (*node.unwrap().as_ptr()).next = nxt;
                    (*nxt.unwrap().as_ptr()).prev = node;
                    self.len += 1;
                }
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < self.len as i32 && index >= 0 {
            if index == 0 {
                self.pop_at_head();
            } else if index == (self.len - 1) as i32 {
                self.pop_at_tail();
            } else {
                let mut tmp = self.head;

                unsafe {
                    for i in 0..index - 1 {
                        tmp = (*tmp.unwrap().as_ptr()).next;
                    }

                    let mut nxt = (*tmp.unwrap().as_ptr()).next.take();
                    nxt = (*nxt.unwrap().as_ptr()).next.take();

                    (*tmp.unwrap().as_ptr()).next = nxt;
                    (*nxt.unwrap().as_ptr()).prev = tmp;

                    self.len -= 1;
                }
            }
        }
    }
}

fn main() {
    let mut linked_list = MyLinkedList::new();
    linked_list.add_at_head(1);
    linked_list.add_at_tail(3);
    linked_list.add_at_index(1, 2); //链表变为1-> 2-> 3
    assert_eq!(linked_list.get(1), 2); //返回2
    linked_list.delete_at_index(1); //现在链表是1-> 3
    assert_eq!(linked_list.get(1), 3); //返回3
}
