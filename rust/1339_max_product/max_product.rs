struct Solution;

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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut best = 0;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node {
                let node_ref = n.borrow();
                *sum += node_ref.val;
                dfs(&node_ref.left, sum);
                dfs(&node_ref.right, sum);
            }
        }

        fn dfs2(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32, best: &mut i32) -> i32 {
            if let Some(n) = node {
                let node_ref = n.borrow();
                let cur = dfs2(&node_ref.left, sum, best)
                    + dfs2(&node_ref.right, sum, best)
                    + node_ref.val;
                if (cur * 2 - sum).abs() < (*best * 2 - sum).abs() {
                    *best = cur;
                }
                cur
            } else {
                0
            }
        }

        dfs(&root, &mut sum);
        dfs2(&root, sum, &mut best);
        ((best as i64) * ((sum - best) as i64) % 1000000007) as i32
    }
}

fn main() {
    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        node(val, None, None)
    }

    let tests = vec![
        // Test 1: [1,2,3,4,5,6]
        (
            node(1, node(2, leaf(4), leaf(5)), node(3, leaf(6), None)),
            110,
        ),
        // Test 2: [1,null,2,3,4,null,null,5,6]
        (
            node(1, None, node(2, leaf(3), node(4, leaf(5), leaf(6)))),
            90,
        ),
        // Test 3: Complex tree
        (
            node(
                2,
                node(3, node(10, leaf(5), leaf(4)), node(7, leaf(11), leaf(1))),
                node(9, leaf(8), leaf(6)),
            ),
            1025,
        ),
        // Test 4: [1,1]
        (node(1, leaf(1), None), 1),
    ];

    for (i, (root, expected)) in tests.into_iter().enumerate() {
        let result = Solution::max_product(root);
        assert_eq!(result, expected, "Test {} failed", i);
        println!("Test {} passed: {}", i, result);
    }
}
