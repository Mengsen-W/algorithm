/*
 * @Date: 2023-08-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-05
 * @FilePath: /algorithm/rust/21_merge_two_lists/merge_two_lists.rs
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut node = None;
        let mut cur = &mut node;
        *cur = loop {
            match (list1, list2) {
                (Some(mut a), Some(mut b)) => {
                    if a.val < b.val {
                        list1 = a.next.take();
                        list2 = Some(b);
                        cur = &mut cur.insert(a).next;
                    } else {
                        list1 = Some(a);
                        list2 = b.next.take();
                        cur = &mut cur.insert(b).next;
                    }
                }
                (x, y) => break x.or(y),
            }
        };
        node
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(4))),
                            })),
                        })),
                    })),
                })),
            })),
        ),
        (None, None, None),
        (
            None,
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(1))),
        ),
    ];

    for (l1, l2, ans) in tests {
        assert_eq!(Solution::merge_two_lists(l1, l2), ans)
    }
}
