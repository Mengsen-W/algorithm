/*
 * @Date: 2023-06-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-12
 * @FilePath: /algorithm/rust/1483_TreeAncestor/TreeAncestor.rs
 */

struct TreeAncestor(Vec<Vec<i32>>);

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl TreeAncestor {
    fn new(_n: i32, mut parent: Vec<i32>) -> Self {
        let mut p = Vec::new();
        p.push(parent);
        let mut flag = true;
        while flag {
            let curr = p.last().unwrap();
            parent = curr
                .iter()
                .map(|&x| *curr.get(x as usize).unwrap_or(&-1))
                .collect();
            flag = parent.iter().any(|&x| x != -1);
            p.push(parent);
        }
        Self(p)
    }

    fn get_kth_ancestor(&self, mut node: i32, k: i32) -> i32 {
        let mut idx = 1;
        for item in &self.0 {
            if idx & k > 0 {
                node = *item.get(node as usize).unwrap_or(&-1);
            }
            idx <<= 1;
            if k < idx {
                return node;
            }
        }
        -1
    }
}

fn main() {
    let parent = vec![-1, 0, 0, 1, 1, 2, 2];
    let t = TreeAncestor::new(3, parent);
    assert_eq!(t.get_kth_ancestor(3, 1), 1);
    assert_eq!(t.get_kth_ancestor(5, 2), 0);
    assert_eq!(t.get_kth_ancestor(6, 3), -1);
}
