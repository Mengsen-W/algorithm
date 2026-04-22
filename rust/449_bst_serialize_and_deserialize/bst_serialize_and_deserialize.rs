/*
 * @Date: 2022-05-11 09:39:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-11 10:00:06
 * @FilePath: /algorithm/449_bst_serialize_and_deserialize/bst_serialize_and_deserialize.rs
 */

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut middle_nums = Vec::new();
        Self::middle_read_tree(&root, &mut middle_nums);
        let res = middle_nums.iter().fold("".to_string(), |mut pre, &pro| {
            pre.push(',');
            pre.push_str(pro.to_string().as_str());
            pre
        });
        res
    }

    fn middle_read_tree(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        let node = root.as_ref();
        if let Some(node) = node {
            nums.push(node.borrow().val);
            Self::middle_read_tree(&node.borrow().left, nums);
            Self::middle_read_tree(&node.borrow().right, nums);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let pre_order = data
            .split(",")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        //中序遍历 是有序的
        let mut middle_order = pre_order.clone();
        middle_order.sort();

        //根据先序遍历和中序遍历构造树即可
        Self::create_tree_from_pre_and_middle(&middle_order, &pre_order)
    }

    fn create_tree_from_pre_and_middle(
        middle_order: &[i32],
        pre_order: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if !middle_order.is_empty() {
            let mut middle_index = 0;
            for i in 0..middle_order.len() {
                if middle_order[i] == pre_order[0] {
                    middle_index = i;
                    break;
                }
            }

            let (middle_left, middle_right) = middle_order.split_at(middle_index);
            let middle_right = &middle_right[1..];

            let (pre_left, pre_right) = pre_order.split_at(middle_index + 1);
            let pre_left = &pre_left[1..];

            let root = Some(Rc::new(RefCell::new(TreeNode {
                val: pre_order[0],
                left: Self::create_tree_from_pre_and_middle(middle_left, pre_left),
                right: Self::create_tree_from_pre_and_middle(middle_right, pre_right),
            })));
            return root;
        }
        None
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        assert_eq!(data, String::from(",2,1,3"));
        let tree = codec.deserialize(data.clone());
        assert_eq!(tree, root);
        println!("serialza = [{:?}] deseralize = [{:?}]", data, tree);
    }
    {
        let root = None;
        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        assert_eq!(data, String::from(""));
        let tree = codec.deserialize(data.clone());
        assert_eq!(tree, root);
        println!("serialza = [{:?}] deseralize = [{:?}]", data, tree);
    }
}
