/*
 * @Date: 2024-02-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-01
 * @FilePath: /algorithm/rust/2581_root_count/root_count.rs
 */

struct Solution;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = edges.len() + 1;
        let mut e = vec![Vec::with_capacity(n); n];
        let mut g: HashSet<(i32, i32)> = HashSet::new();
        let mut ans = 0;
        let cnt0 = Rc::new(RefCell::new(0));
        for v in edges.iter() {
            e[v[0] as usize].push(v[1]);
            e[v[1] as usize].push(v[0]);
        }
        for v in guesses.iter() {
            g.insert((v[0], v[1]));
        }
        fn dfs(
            x: i32,
            fa: i32,
            e: &Vec<Vec<i32>>,
            g: &HashSet<(i32, i32)>,
            cnt0: &Rc<RefCell<i32>>,
        ) {
            for y in e[x as usize].iter() {
                if fa != *y {
                    if g.contains(&(x, *y)) {
                        *cnt0.borrow_mut() += 1;
                    }
                    dfs(*y, x, e, g, cnt0);
                }
            }
        }
        dfs(0, -1, &e, &g, &cnt0);
        let t = cnt0.take();
        fn root(
            x: i32,
            fa: i32,
            e: &Vec<Vec<i32>>,
            g: &HashSet<(i32, i32)>,
            cnt: i32,
            k: i32,
            ans: &mut i32,
        ) {
            if cnt >= k {
                *ans += 1;
            }
            for y in e[x as usize].iter() {
                if fa != *y {
                    let mut u = cnt;
                    if g.contains(&(x, *y)) {
                        u -= 1;
                    }
                    if g.contains(&(*y, x)) {
                        u += 1;
                    }
                    root(*y, x, e, g, u, k, ans);
                }
            }
        }
        root(0, -1, &e, &g, t, k, &mut ans);
        return ans;
    }
}

fn main() {
    let tests = vec![
        (
            [[0, 1], [1, 2], [1, 3], [4, 2]],
            [[1, 3], [0, 1], [1, 0], [2, 4]],
            3,
            3,
        ),
        (
            [[0, 1], [1, 2], [2, 3], [3, 4]],
            [[1, 0], [3, 4], [2, 1], [3, 2]],
            1,
            5,
        ),
    ];

    for (edges, guesses, k, ans) in tests {
        assert_eq!(
            Solution::root_count(
                edges.iter().map(|v| v.to_vec()).collect(),
                guesses.iter().map(|v| v.to_vec()).collect(),
                k
            ),
            ans
        );
    }
}
