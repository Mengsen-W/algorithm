/*
 * @Date: 2023-02-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-08
 * @FilePath: /algorithm/rust/1223_remove_subfolders/remove_subfolders.rs
 */

struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut trie = TrieNode::<&str>::new();
        folder.iter().for_each(|s| {
            let v: Vec<&str> = s.split("/").collect();
            trie.insert(&v);
        });

        let mut v = vec![];
        folder.iter().for_each(|s| {
            let vec: Vec<&str> = s.split("/").collect();
            if !trie.is_sub_dir(&vec) {
                v.push(s.clone());
            }
        });

        v
    }
}

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
struct TrieNode<T: Eq + Debug + Hash + Clone> {
    is_end: bool,
    children: HashMap<T, TrieNode<T>>,
}

impl<T: Eq + Debug + Hash + Clone> TrieNode<T> {
    fn new() -> TrieNode<T> {
        TrieNode {
            is_end: false,
            children: HashMap::new(),
        }
    }

    fn insert_inner(&mut self, vec: &Vec<T>, index: usize) {
        if index < vec.len() {
            let value = vec[index].clone();
            if self.children.get_mut(&value).is_some() {
                self.children
                    .get_mut(&value)
                    .unwrap()
                    .insert_inner(vec, index + 1)
            } else {
                let mut node = TrieNode::new();
                node.insert_inner(vec, index + 1);
                self.children.insert(value, node);
            }
        } else {
            self.is_end = true;
        }
    }

    fn insert(&mut self, vec: &Vec<T>) {
        self.insert_inner(vec, 0);
    }

    fn is_sub_dir(&self, vec: &Vec<T>) -> bool {
        let mut node = self;
        let mut result = false;
        for (index, item) in vec.iter().enumerate() {
            let n = node.children.get(item);
            if n.is_some() {
                node = n.unwrap();
                if node.is_end && index != vec.len() - 1 {
                    result = true;
                    break;
                }
            } else {
                break;
            }
        }
        result
    }
}

fn main() {
    {
        let folder: Vec<String> = vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["/a", "/c/d", "/c/f"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::remove_subfolders(folder), ans);
    }

    {
        let folder: Vec<String> = vec!["/a", "/a/b/c", "/a/b/d"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["/a"].iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::remove_subfolders(folder), ans);
    }

    {
        let folder: Vec<String> = vec!["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = vec!["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::remove_subfolders(folder), ans);
    }
}
