/*
 * @Date: 2023-09-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-23
 * @FilePath: /algorithm/rust/1993_LockingTree/LockingTree.rs
 */

struct LockingTree {
    parent: Vec<usize>,
    locked_by: Vec<Option<i32>>,
    children: Vec<Vec<usize>>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let parent: Vec<_> = parent.into_iter().map(|x| x as usize).collect();
        let n = parent.len();
        let mut children = vec![vec![]; n];
        for i in 1..n {
            children[parent[i]].push(i);
        }
        Self {
            parent,
            locked_by: vec![None; n],
            children,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        match self.locked_by[num] {
            None => {
                self.locked_by[num] = Some(user);
                true
            }
            Some(_) => false,
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        match self.locked_by[num] {
            Some(u) if u == user => {
                self.locked_by[num] = None;
                true
            }
            _ => false,
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        if self.locked_by[num].is_some() {
            return false;
        }
        let mut p = self.parent[num];
        while p != usize::MAX {
            if self.locked_by[p].is_some() {
                return false;
            }
            p = self.parent[p];
        }
        let mut cd = self.children[num].clone();
        let mut res = false;
        while let Some(desc) = cd.pop() {
            if self.locked_by[desc].is_some() {
                self.locked_by[desc] = None;
                res = true;
            }
            cd.extend(self.children[desc].iter());
        }
        if res {
            self.locked_by[num] = Some(user);
        }
        res
    }
}

fn main() {
    let mut locking_tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2]);
    assert_eq!(locking_tree.lock(2, 2), true);
    assert_eq!(locking_tree.unlock(2, 3), false);
    assert_eq!(locking_tree.unlock(2, 2), true);
    assert_eq!(locking_tree.lock(4, 5), true);
    assert_eq!(locking_tree.upgrade(0, 1), true);
    assert_eq!(locking_tree.upgrade(0, 1), false);
}
