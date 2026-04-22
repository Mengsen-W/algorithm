/*
 * @Date: 2024-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-02
 * @FilePath: /algorithm/rust/2368_reachable_nodes/reachable_nodes.rs
 */

struct Solution;

use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let cant_use: HashSet<i32> = HashSet::from_iter(restricted.into_iter());
        let mut res = 0;

        let used: RefCell<HashSet<i32>> = RefCell::new(HashSet::with_capacity(n as usize));
        let mut tree: Vec<Vec<i32>> = vec![vec![]; (n + 1) as usize];
        for i in edges {
            tree[i[0] as usize].push(i[1]);
            tree[i[1] as usize].push(i[0]);
        }
        //let tree = Rc::new(RefCell::new(tree));

        Self::dfs(&mut res, &0, &used, &tree, &cant_use);
        res
    }
    fn dfs(
        res: &mut i32,
        root: &i32,
        used: &RefCell<HashSet<i32>>,
        tree: &Vec<Vec<i32>>,
        cant_use: &HashSet<i32>,
    ) {
        {
            if used.borrow().contains(&root) {
                return;
            }
            used.borrow_mut().insert(*root);
        }
        *res += 1;

        for item in tree[*root as usize].iter() {
            if cant_use.contains(item) {
                continue;
            }
            Self::dfs(res, item, used, tree, cant_use);
        }
    }
}

fn main() {
    let tests = vec![
        (
            7,
            vec![[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]],
            vec![4, 5],
            4,
        ),
        (
            7,
            vec![[0, 1], [0, 2], [0, 5], [0, 4], [3, 2], [6, 5]],
            vec![4, 2, 1],
            3,
        ),
    ];

    for (n, edges, restricted, ans) in tests {
        assert_eq!(
            Solution::reachable_nodes(n, edges.iter().map(|v| v.to_vec()).collect(), restricted),
            ans
        );
    }
}
