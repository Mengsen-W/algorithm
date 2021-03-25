/*
 * @Date: 2021-03-25 09:25:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-25 09:54:01
 */

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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut node = dummy.as_mut();
    while node.as_ref().unwrap().next.is_some() {
        let mut ptr = node.as_mut().unwrap().next.as_mut();
        let val = ptr.as_mut().unwrap().val;
        let mut dup = false;
        while ptr.as_mut().unwrap().next.is_some()
            && ptr.as_mut().unwrap().next.as_mut().unwrap().val == val
        {
            ptr = ptr.unwrap().next.as_mut();
            dup = true;
        }
        if dup {
            node.as_mut().unwrap().next = ptr.unwrap().next.take();
        } else {
            node = node.unwrap().next.as_mut();
        }
    }
    dummy.unwrap().next
}

fn main() {
    let head = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    };
    let cur = delete_duplicates(Some(Box::new(head)));
    println!("{:#?}", cur);

    let head = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    };
    let cur = delete_duplicates(Some(Box::new(head)));
    println!("{:#?}", cur);
}
