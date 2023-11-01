/*
 * @Date: 2023-11-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-01
 * @FilePath: /algorithm/rust/2127_maximum_invitations/maximum_invitations.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        let n = favorite.len();
        let mut deg = vec![0; n];
        for &f in &favorite {
            deg[f as usize] += 1;
        }
        let mut max_depth = vec![1; n];
        let mut q = VecDeque::new();
        for (i, &d) in deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        while let Some(x) = q.pop_front() {
            let y = favorite[x] as usize;
            max_depth[y] = max_depth[x] + 1;
            deg[y] -= 1;
            if deg[y] == 0 {
                q.push_back(y);
            }
        }
        let mut max_ring_size = 0;
        let mut sum_chain_size = 0;
        for i in 0..n {
            if deg[i] == 0 {
                continue;
            }
            deg[i] = 0;
            let mut ring_size = 1; // 基环长度
            let mut x = favorite[i] as usize;
            while x != i {
                deg[x] = 0;
                ring_size += 1;
                x = favorite[x] as usize;
            }
            if ring_size == 2 {
                sum_chain_size += max_depth[i] + max_depth[favorite[i] as usize];
            } else {
                max_ring_size = max_ring_size.max(ring_size); // 取所有基环长度的最大值
            }
        }
        max_ring_size.max(sum_chain_size)
    }
}

fn main() {
    let tests = vec![
        (vec![2, 2, 1, 2], 3),
        (vec![1, 2, 0], 3),
        (vec![3, 0, 1, 4, 1], 4),
    ];

    for (favorite, ans) in tests {
        assert_eq!(Solution::maximum_invitations(favorite), ans);
    }
}
