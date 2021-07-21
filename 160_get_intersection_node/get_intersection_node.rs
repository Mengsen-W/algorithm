/*
 * @Date: 2021-06-04 19:17:21
 * @Author: mengsenwang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-21 20:13:57
 */

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

fn get_intersection_node_hash(
    a: Option<Rc<RefCell<ListNode>>>,
    b: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    use std::collections::HashSet;
    let mut visited: HashSet<Rc<RefCell<ListNode>>> = HashSet::new();
    let mut temp = a;
    while let Some(ref i) = temp {
        // visited.insert(i);
        let temp: Option<Rc<RefCell<ListNode>>> = temp.borrow().next.clone();
        // temp = temp.next;
    }
    // println!("{:?}", visited);
    // temp = b;
    // while temp != None {
    //     if visited.contains(&temp.unwrap().as_ref()) {
    //         return temp;
    //     }
    //     temp = (*(temp.unwrap().as_ref())).next;
    // }
    None
}

fn print_list(l: Option<Rc<RefCell<ListNode>>>) {
    for node in l.iter() {
        println!("{:?}", node);
    }
}

fn main() {
    {
        let intersect = Some(Rc::new(RefCell::new(ListNode {
            val: 8,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 4,
                next: Some(Rc::new(RefCell::new(ListNode { val: 5, next: None }))),
            }))),
        })));
        let a = Some(Rc::new(RefCell::new(ListNode {
            val: 4,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 1,
                next: Some(Rc::clone(&(intersect.as_ref().unwrap()))),
            }))),
        })));
        let b = Some(Rc::new(RefCell::new(ListNode {
            val: 5,
            next: Some(Rc::new(RefCell::new(ListNode {
                val: 0,
                next: Some(Rc::new(RefCell::new(ListNode {
                    val: 1,
                    next: Some(Rc::clone(&(intersect.as_ref().unwrap()))),
                }))),
            }))),
        })));
        get_intersection_node_hash(a, b);
        // print_list(a);
    }
}
