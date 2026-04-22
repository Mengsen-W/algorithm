/*
 * @Date: 2024-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-05
 * @FilePath: /algorithm/rust/2912_get_ancestors/get_ancestors.rs
 */

struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        // 存储每个节点的祖先节点的辅助数组
        let mut anc: Vec<HashSet<i32>> = vec![HashSet::new(); n];
        // 邻接表
        let mut e: Vec<Vec<i32>> = vec![Vec::new(); n];
        // 入度表
        let mut indeg: Vec<i32> = vec![0; n];

        // 预处理
        for edge in edges {
            e[edge[0] as usize].push(edge[1]);
            indeg[edge[1] as usize] += 1;
        }

        // 广度优先搜索求解拓扑排序
        let mut q: VecDeque<i32> = VecDeque::new();
        for i in 0..n {
            if indeg[i] == 0 {
                q.push_back(i as i32);
            }
        }
        while let Some(u) = q.pop_front() {
            for v in &e[u as usize] {
                // 复制祖先节点集合，避免同时遍历和修改
                let mut new_ancestors = anc[*v as usize].clone();
                // 更新子节点的祖先节点集合
                new_ancestors.insert(u);
                for i in &anc[u as usize] {
                    new_ancestors.insert(*i);
                }
                anc[*v as usize] = new_ancestors;
                indeg[*v as usize] -= 1;
                if indeg[*v as usize] == 0 {
                    q.push_back(*v);
                }
            }
        }

        // 转化为答案数组
        let mut res: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        for i in 0..n as usize {
            res[i] = anc[i].iter().cloned().collect::<Vec<i32>>();
            res[i].sort();
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            8,
            vec![
                vec![0, 3],
                vec![0, 4],
                vec![1, 3],
                vec![2, 4],
                vec![2, 7],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![4, 6],
            ],
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3],
            ],
        ),
        (
            5,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![0, 4],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]],
        ),
    ];

    for (n, edges, ans) in tests {
        assert_eq!(Solution::get_ancestors(n, edges), ans);
    }
}
