/*
 * @Date: 2022-10-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-12
 * @FilePath: /algorithm/817_num_components/num_components.rs
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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut set: Vec<bool> = vec![false; 10001];
        for i in nums {
            set[i as usize] = true;
        }
        let mut prev_comp = false;
        let mut result = 0;
        let mut head = &head;
        while let Some(node) = head {
            if set[node.val as usize] {
                if !prev_comp {
                    result += 1;
                    prev_comp = true;
                }
            } else {
                prev_comp = false;
            }
            head = &node.next;
        }
        result
    }
}

fn main() {
    {
        let header = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        let nums = vec![0, 1, 3];
        let ans = 2;
        assert_eq!(Solution::num_components(header, nums), ans);
    }

    {
        let header = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            })),
        }));
        let nums = vec![0, 3, 1, 4];
        let ans = 2;
        assert_eq!(Solution::num_components(header, nums), ans);
    }
}
