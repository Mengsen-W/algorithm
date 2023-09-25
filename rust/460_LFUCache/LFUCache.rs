/*
 * @Date: 2023-09-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-25
 * @FilePath: /algorithm/rust/460_LFUCache/LFUCache.rs
 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    freq: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            freq: 1,
            prev: None,
            next: None,
        }))
    }
}

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
    freq_to_dummy: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_node: HashMap::new(),
            freq_to_dummy: HashMap::new(),
        }
    }

    fn get_node(&mut self, key: i32) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            self.remove(node.clone()); // 把这本书抽出来
            let freq = node.borrow().freq;
            let dummy = self.freq_to_dummy.get(&freq).unwrap();
            if Rc::ptr_eq(dummy, dummy.borrow().prev.as_ref().unwrap()) {
                // 抽出来后，这摞书是空的
                self.freq_to_dummy.remove(&freq); // 移除空链表
                if self.min_freq == freq {
                    self.min_freq += 1;
                }
            }
            node.borrow_mut().freq += 1;
            self.push_front(freq + 1, node.clone()); // 放在右边这摞书的最上面
            return Some(node);
        }
        None // 没有这本书
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.get_node(key) {
            // 有这本书
            return node.borrow().value;
        }
        -1 // 没有这本书
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.get_node(key) {
            // 有这本书
            node.borrow_mut().value = value; // 更新 value
            return;
        }
        if self.key_to_node.len() == self.capacity {
            // 书太多了
            let dummy = self.freq_to_dummy.get(&self.min_freq).unwrap();
            let dummy2 = &dummy.clone();
            let back_node = dummy.borrow().prev.clone().unwrap(); // 最左边那摞书的最下面的书
            let key = back_node.borrow().key;
            self.key_to_node.remove(&key);
            self.remove(back_node); // 移除
            if Rc::ptr_eq(dummy2, dummy2.borrow().prev.as_ref().unwrap()) {
                // 抽出来后，这摞书是空的
                self.freq_to_dummy.remove(&self.min_freq); // 移除空链表
            }
        }
        let node = Node::new(key, value); // 新书
        self.key_to_node.insert(key, node.clone());
        self.push_front(1, node.clone()); // 放在「看过 1 次」的最上面
        self.min_freq = 1;
    }

    // 创建一个新的双向链表
    fn new_list() -> Rc<RefCell<Node>> {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        dummy
    }

    // 在链表头添加一个节点（把一本书放在最上面）
    fn push_front(&mut self, freq: i32, x: Rc<RefCell<Node>>) {
        let dummy = self
            .freq_to_dummy
            .entry(freq)
            .or_insert_with(|| Self::new_list());
        let next = dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(dummy.clone());
        x.borrow_mut().next = next.clone();
        dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x);
    }

    // 删除一个节点（抽出一本书）
    fn remove(&mut self, x: Rc<RefCell<Node>>) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }
}

fn main() {
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1); // cache=[1,_], cnt(1)=1
    lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
    assert_eq!(lfu.get(1), 1); // 返回 1
                               // cache=[1,2], cnt(2)=1, cnt(1)=2
    lfu.put(3, 3); // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
                   // cache=[3,1], cnt(3)=1, cnt(1)=2
    assert_eq!(lfu.get(2), -1); // 返回 -1（未找到）
    assert_eq!(lfu.get(3), 3); // 返回 3
                               // cache=[3,1], cnt(3)=2, cnt(1)=2
    lfu.put(4, 4); // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
                   // cache=[4,3], cnt(4)=1, cnt(3)=2
    assert_eq!(lfu.get(1), -1); // 返回 -1（未找到）
    assert_eq!(lfu.get(3), 3); // 返回 3
                               // cache=[3,4], cnt(4)=1, cnt(3)=3
    assert_eq!(lfu.get(4), 4); // 返回 4
                               // cache=[3,4], cnt(4)=2, cnt(3)=3
}
