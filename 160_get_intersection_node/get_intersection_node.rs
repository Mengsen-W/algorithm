/*
 * @Date: 2021-06-04 19:17:21
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-06-04 20:27:32
 */

use std::cell::RefCell;
use std::rc::Rc;

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

fn get_intersection_node_hash(
    a: Option<Rc<RefCell<ListNode>>>,
    b: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    let mut temp = a;
    while temp != None {
        visited.insert(temp);
        temp = temp.next;
    }
    temp = b;
    while temp != None {
        if visited.contains(temp) {
            return temp;
        }
        temp = temp.next;
    }
    None
}

fn main() {
    {
        let intersect = Rc::new(RefCell::new(ListNode {
            val: 8,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 4,
                next: Some(Rc::new(RefCell::new(ListNode { val: 5, next: None }))),
            }))),
        }));
        let a = Some(Rc::new(RefCell::new(ListNode {
            val: 4,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 1,
                next: Some(Rc::clone(&intersect)),
            }))),
        })));
        let b = Some(Rc::new(RefCell::new(ListNode {
            val: 5,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 0,
                next: Some(Rc::new(RefCell::new(ListNode {
                    val: 1,
                    next: Some(Rc::clone(&intersect)),
                }))),
            }))),
        })));
        get_intersection_node_hash(a, b);
    }
}
