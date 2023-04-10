/*
 * @Date: 2023-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-10
 * @FilePath: /algorithm/rust/1019_next_larger_nodes/next_larger_nodes.rs
 */

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

struct Solution;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stk: Vec<(i32, i32)> = Vec::new();
        let mut result = Vec::new();

        let mut curr = head;
        let mut i = 0;
        while let Some(mut curr_node) = curr {
            result.push(0);
            while !stk.is_empty() && curr_node.val > stk.last().unwrap().0 {
                let index = stk.last().unwrap().1;
                result[index as usize] = curr_node.val;
                stk.pop();
            }
            stk.push((curr_node.val, i as i32));
            curr = curr_node.next.take();
            i += 1;
        }
        result
    }
}

fn main() {
    {
        let head = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(5))),
            })),
        }));
        let ans = vec![5, 5, 0];
        assert_eq!(Solution::next_larger_nodes(head), ans);
    }

    {
        let head = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let ans = vec![7, 0, 5, 5, 0];
        assert_eq!(Solution::next_larger_nodes(head), ans);
    }
}
