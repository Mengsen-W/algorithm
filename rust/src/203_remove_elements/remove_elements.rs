/*
 * @Date: 2021-06-05 09:12:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-05 10:49:01
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

fn remove_elements_iteration(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode { val: 0, next: None };
    let mut cur = &mut dummy_head;
    while let Some(mut node) = head {
        head = std::mem::replace(&mut node.next, None);
        if node.val != val {
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}

fn main() {
    // use std::rc::Rc;
    // let a = Rc::new(1);
    // let b = a.clone();
    // println!("{:?}", Rc::into_raw(a));
    // println!("{:?}", Rc::into_raw(b));

    println!(
        "{:?}",
        remove_elements_iteration(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            6
        )
    );
}
