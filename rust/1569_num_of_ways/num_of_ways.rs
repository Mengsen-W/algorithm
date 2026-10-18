struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const MOD: i64 = 1000000007;

#[derive(Debug)]
struct TNode {
    left: Option<Rc<RefCell<TNode>>>,
    right: Option<Rc<RefCell<TNode>>>,
    size: i32,
    ans: i64,
}

impl TNode {
    fn new() -> Self {
        TNode {
            left: None,
            right: None,
            size: 1,
            ans: 0,
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    root: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            root: (0..n).collect(),
        }
    }

    fn findset(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.findset(self.parent[x]);
        }
        self.parent[x]
    }

    fn getroot(&mut self, x: usize) -> usize {
        let root_idx = self.findset(x);
        self.root[root_idx]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root[x];
        let root_y = self.root[y];
        self.root[y] = root_x;

        let mut x = x;
        let mut y = y;
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
    }

    fn find_and_unite(&mut self, x: usize, y: usize) -> bool {
        let x0 = self.findset(x);
        let y0 = self.findset(y);
        if x0 != y0 {
            self.unite(x0, y0);
            return true;
        }
        false
    }
}

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let mut fac = vec![0i64; n];
        let mut inv = vec![0i64; n];
        let mut fac_inv = vec![0i64; n];
        fac[0] = 1;
        inv[0] = 1;
        fac_inv[0] = 1;
        if n > 1 {
            fac[1] = 1;
            inv[1] = 1;
            fac_inv[1] = 1;
        }
        for i in 2..n {
            fac[i] = (fac[i - 1] * i as i64) % MOD;
            inv[i] = ((MOD - MOD / i as i64) * inv[(MOD % i as i64) as usize]) % MOD;
            fac_inv[i] = (fac_inv[i - 1] * inv[i]) % MOD;
        }

        let mut found: HashMap<usize, Rc<RefCell<TNode>>> = HashMap::new();
        let mut uf = UnionFind::new(n);

        for i in (0..n).rev() {
            let val = (nums[i] - 1) as usize;
            let node = Rc::new(RefCell::new(TNode::new()));

            if val > 0 && found.contains_key(&(val - 1)) {
                let lchild = uf.getroot(val - 1);
                if let Some(left_node) = found.get(&lchild) {
                    node.borrow_mut().left = Some(left_node.clone());
                    let left_size = left_node.borrow().size;
                    node.borrow_mut().size += left_size;
                    uf.find_and_unite(val, lchild);
                }
            }

            if val < n - 1 && found.contains_key(&(val + 1)) {
                let rchild = uf.getroot(val + 1);
                if let Some(right_node) = found.get(&rchild) {
                    node.borrow_mut().right = Some(right_node.clone());
                    let right_size = right_node.borrow().size;
                    node.borrow_mut().size += right_size;
                    uf.find_and_unite(val, rchild);
                }
            }

            let lsize = node.borrow().left.as_ref().map_or(0, |n| n.borrow().size);
            let rsize = node.borrow().right.as_ref().map_or(0, |n| n.borrow().size);
            let lans = node.borrow().left.as_ref().map_or(1, |n| n.borrow().ans);
            let rans = node.borrow().right.as_ref().map_or(1, |n| n.borrow().ans);

            let ans = (fac[(lsize + rsize) as usize] * fac_inv[lsize as usize] % MOD
                * fac_inv[rsize as usize]
                % MOD
                * lans
                % MOD
                * rans
                % MOD)
                % MOD;

            node.borrow_mut().ans = ans;
            found.insert(val, node);
        }

        let root_key = (nums[0] - 1) as usize;
        let root = found.get(&root_key).unwrap();
        ((root.borrow().ans - 1 + MOD) % MOD) as i32
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 3], 1),
        (vec![3, 4, 5, 1, 2], 5),
        (vec![1, 2, 3], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::num_of_ways(nums), ans);
    }
}
