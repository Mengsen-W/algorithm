use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut is_root: HashMap<i32, bool> = HashMap::new(); // 数值对应的节点是否为根节点的哈希表
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new(); // 数值与对应节点的哈希表

        for d in descriptions {
            let p = d[0];
            let c = d[1];
            let left = d[2] == 1;

            if !is_root.contains_key(&p) {
                is_root.insert(p, true);
            }
            is_root.insert(c, false);

            // 创建或更新节点
            if !nodes.contains_key(&p) {
                nodes.insert(p, Rc::new(RefCell::new(TreeNode::new(p))));
            }
            if !nodes.contains_key(&c) {
                nodes.insert(c, Rc::new(RefCell::new(TreeNode::new(c))));
            }

            let parent = nodes.get(&p).unwrap().clone();
            let child = nodes.get(&c).unwrap().clone();

            if left {
                parent.borrow_mut().left = Some(child);
            } else {
                parent.borrow_mut().right = Some(child);
            }
        }

        // 寻找根节点
        let mut root_val = -1;
        for (val, is_root_node) in is_root {
            if is_root_node {
                root_val = val;
                break;
            }
        }

        nodes.get(&root_val).cloned()
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![20, 15, 1],
                vec![20, 17, 0],
                vec![50, 20, 1],
                vec![50, 80, 0],
                vec![80, 19, 1],
            ],
            Some(Rc::new(RefCell::new(TreeNode {
                val: 50,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 80,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(19)))),
                    right: None,
                }))),
            }))),
        ),
        (
            vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]],
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
        ),
    ];

    for (descriptions, expected) in tests {
        assert_eq!(Solution::create_binary_tree(descriptions), expected);
    }
}
