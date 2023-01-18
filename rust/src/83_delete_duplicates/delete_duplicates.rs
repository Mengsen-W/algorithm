/*
 * @Date: 2021-03-26 08:49:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-26 09:21:43
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
    let mut head = head;
    let mut ptr = head.as_mut();
    while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
        let cur_val = ptr.as_ref().unwrap().val;
        let next_val = ptr.as_ref().unwrap().next.as_ref().unwrap().val;
        if cur_val == next_val {
            let mut next = ptr.as_mut().unwrap().next.as_mut().unwrap().next.take();
            ptr.as_mut().unwrap().next = next;
        } else {
            ptr = ptr.unwrap().next.as_mut();
        }
    }
    head
}

fn main() {
    let head = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    };
    let cur = delete_duplicates(Some(Box::new(head)));
    println!("{:?}", cur);

    let head = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    };
    let cur = delete_duplicates(Some(Box::new(head)));
    println!("{:?}", cur);
}
