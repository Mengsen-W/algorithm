/*
 * @Date: 2023-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-30
 * @FilePath: /algorithm/rust/1669_merge_in_between/merge_in_between.rs
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_in_between(
    mut list1: Option<Box<ListNode>>,
    a: i32,
    b: i32,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut n1 = &mut list1;
    for _ in 1..a {
        n1 = &mut n1.as_mut().unwrap().next;
    }
    let mut bak = n1.as_mut().unwrap().next.take();
    n1.as_mut().unwrap().next = list2;
    while n1.as_ref().unwrap().next.is_some() {
        n1 = &mut n1.as_mut().unwrap().next;
    }
    let mut n2 = &mut bak;
    for _ in a..=b {
        n2 = &mut n2.as_mut().unwrap().next;
    }
    n1.as_mut().unwrap().next = n2.take();
    list1
}

fn main() {
    {
        let list1 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1000000,
            next: Some(Box::new(ListNode {
                val: 1000001,
                next: Some(Box::new(ListNode {
                    val: 1000002,
                    next: None,
                })),
            })),
        }));
        let a = 3;
        let b = 4;
        let ans = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1000000,
                        next: Some(Box::new(ListNode {
                            val: 1000001,
                            next: Some(Box::new(ListNode {
                                val: 1000002,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_in_between(list1, a, b, list2), ans);
    }

    {
        let list1 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1000000,
            next: Some(Box::new(ListNode {
                val: 1000001,
                next: Some(Box::new(ListNode {
                    val: 1000002,
                    next: Some(Box::new(ListNode {
                        val: 1000003,
                        next: Some(Box::new(ListNode {
                            val: 1000004,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let a = 2;
        let b = 5;
        let ans = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1000000,
                    next: Some(Box::new(ListNode {
                        val: 1000001,
                        next: Some(Box::new(ListNode {
                            val: 1000002,
                            next: Some(Box::new(ListNode {
                                val: 1000003,
                                next: Some(Box::new(ListNode {
                                    val: 1000004,
                                    next: Some(Box::new(ListNode { val: 6, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_in_between(list1, a, b, list2), ans);
    }
}
