/*
 * @Date: 2023-09-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-24
 * @FilePath: /algorithm/rust/146_LRUCache/LRUCache.rs
 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        LRUCache {
            capacity: capacity as usize,
            dummy,
            key_to_node: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            let value = node.borrow().value;
            self.remove(node.clone()); // 把这本书抽出来
            self.push_front(node); // 放在最上面
            return value;
        }
        -1 // 没有这本书
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            node.borrow_mut().value = value; // 更新 value
            self.remove(node.clone()); // 把这本书抽出来
            self.push_front(node); // 放在最上面
            return;
        }
        let node = Node::new(key, value); // 新书
        self.key_to_node.insert(key, node.clone());
        self.push_front(node); // 放在最上面
        if self.key_to_node.len() > self.capacity {
            // 书太多了
            let back_node = self.dummy.borrow().prev.clone().unwrap();
            self.key_to_node.remove(&back_node.borrow().key);
            self.remove(back_node); // 去掉最后一本书
        }
    }

    // 删除一个节点（抽出一本书）
    fn remove(&mut self, x: Rc<RefCell<Node>>) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    // 在链表头添加一个节点（把一本书放在最上面）
    fn push_front(&mut self, x: Rc<RefCell<Node>>) {
        let next = self.dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(self.dummy.clone());
        x.borrow_mut().next = next.clone();
        self.dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x);
    }
}

fn main() {
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    assert_eq!(lru_cache.get(1), 1);
    lru_cache.put(3, 3);
    assert_eq!(lru_cache.get(2), -1);
    lru_cache.put(4, 4);
    assert_eq!(lru_cache.get(1), -1);
    assert_eq!(lru_cache.get(3), 3);
    assert_eq!(lru_cache.get(4), 4);
}
