/*
 * @Date: 2021-05-16 09:51:54
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-04
 */

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug)]
struct Trie<T> {
    root: Rc<RefCell<TrieNode<T>>>,
}

#[derive(Debug, Default)]
struct TrieNode<T> {
    left: Option<Rc<RefCell<TrieNode<T>>>>,
    right: Option<Rc<RefCell<TrieNode<T>>>>,
}

impl Trie<i32> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(TrieNode::<i32>::default())),
        }
    }
    pub fn add(&mut self, x: i32) {
        let f = || Rc::new(RefCell::new(TrieNode::<i32>::default()));
        let mut p = self.root.clone();
        (0..31).rev().for_each(|k| {
            p = {
                let mut bor = p.borrow_mut();
                if x & (1 << k) == 0 {
                    bor.left.get_or_insert_with(f)
                } else {
                    bor.right.get_or_insert_with(f)
                }
                .clone()
            };
        });
    }
    pub fn get_max(&self, x: i32) -> i32 {
        let mut p = self.root.clone();
        let mut m = 0;
        (0..31).rev().for_each(|k| {
            p = {
                let bor = p.borrow();
                let (a, b) = if x & (1 << k) == 0 {
                    (bor.right.as_ref(), bor.left.as_ref())
                } else {
                    (bor.left.as_ref(), bor.right.as_ref())
                };
                if let Some(a) = a {
                    m |= 1 << k;
                    a.clone()
                } else if let Some(b) = b {
                    b.clone()
                } else {
                    unreachable!()
                }
            };
        });
        m
    }
}

struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut t = Trie::new();
        nums.into_iter().fold(0, |ans, x| {
            t.add(x);
            max(ans, t.get_max(x))
        })
    }
}

fn main() {
    let tests = vec![
        (vec![2, 10, 5, 25, 2, 8], 28),
        (vec![0], 0),
        (vec![2, 4], 6),
        (vec![8, 10, 2], 10),
        (vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70], 127),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_maximum_xor(nums), ans);
    }
}
