struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (head.clone(), root.clone()) {
            (None, _) => true,
            (_, None) => false,
            (Some(list_node), Some(tree_node)) => {
                Self::dfs(&head, &root)
                    || Self::is_sub_path(head.clone(), tree_node.borrow().left.clone())
                    || Self::is_sub_path(head.clone(), tree_node.borrow().right.clone())
            }
        }
    }

    pub fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (head, root) {
            (None, _) => true,  // 链表已经全部匹配完，匹配成功
            (_, None) => false, // 二叉树访问到了空节点，匹配失败
            (Some(list_node), Some(tree_node)) => {
                list_node.val == tree_node.borrow().val && // 当前匹配的二叉树上节点的值与链表节点的值不相等，匹配失败
                (Self::dfs(&list_node.next, &tree_node.borrow().left) || Self::dfs(&list_node.next, &tree_node.borrow().right))
            }
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        }))),
                    }))),
                    right: None,
                }))),
            }))),
            true,
        ),
        (
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        }))),
                    }))),
                    right: None,
                }))),
            }))),
            true,
        ),
        (
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        }))),
                    }))),
                    right: None,
                }))),
            }))),
            false,
        ),
    ];

    for (head, root, ans) in tests {
        assert_eq!(Solution::is_sub_path(head, root), ans);
    }
}
