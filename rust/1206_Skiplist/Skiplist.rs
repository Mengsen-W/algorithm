/*
 * @Date: 2022-07-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-26
 * @FilePath: /algorithm/1206_Skiplist/Skiplist.rs
 */

struct Skiplist {
    d: Vec<i32>,
}

impl Skiplist {
    fn new() -> Self {
        Self { d: vec![0; 20001] }
    }

    fn search(&self, target: i32) -> bool {
        self.d[target as usize] > 0
    }

    fn add(&mut self, num: i32) {
        self.d[num as usize] += 1;
    }

    fn erase(&mut self, num: i32) -> bool {
        match self.d[num as usize] {
            0 => false,
            _ => {
                self.d[num as usize] -= 1;
                true
            }
        }
    }
}

fn main() {
    let mut s = Skiplist::new();
    s.add(1);
    s.add(2);
    s.add(3);
    assert!(!s.search(0));
    s.add(4);
    assert!(s.search(1));
    assert!(!s.erase(0));
    assert!(s.erase(1));
    assert!(!s.search(1));
}
