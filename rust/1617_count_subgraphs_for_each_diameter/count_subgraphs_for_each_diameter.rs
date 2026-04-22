/*
 * @Date: 2023-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-12
 * @FilePath: /algorithm/rust/1617_count_subgraphs_for_each_diameter/count_subgraphs_for_each_diameter.rs
 */

struct Solution;

use std::cmp::max;
use std::collections::VecDeque;
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize + 1];
        Self::build_graph(&mut graph, &edges);
        let mut ans = vec![0; n as usize - 1];
        for i in 1..(1 << n) {
            let nodes = i as i32;
            if nodes.count_ones() > 1 {
                if Self::check_connectivity(nodes, &graph) {
                    let dist = Self::max_dist(nodes, &graph);
                    ans[dist as usize - 1] += 1;
                }
            }
        }
        ans
    }

    fn build_graph(graph: &mut Vec<Vec<i32>>, edges: &Vec<Vec<i32>>) {
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
    }

    fn check_connectivity(nodes: i32, graph: &Vec<Vec<i32>>) -> bool {
        let root = nodes.trailing_zeros() as i32 + 1;
        let node_cnt = nodes.count_ones();
        let mut visited = Self::mark_visited(0, root);

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            for &next in graph[node as usize].iter() {
                if !Self::is_visited(visited, next) && Self::contain_node(nodes, next) {
                    visited = Self::mark_visited(visited, next);
                    queue.push_back(next);
                }
            }
        }
        return node_cnt == visited.count_ones();
    }

    fn contain_node(nodes: i32, node_id: i32) -> bool {
        return Self::is_visited(nodes, node_id);
    }

    fn is_visited(nodes: i32, node_id: i32) -> bool {
        return nodes & (1 << (node_id - 1)) != 0;
    }

    fn mark_visited(nodes: i32, node_id: i32) -> i32 {
        return nodes | (1 << (node_id - 1));
    }

    fn max_dist(nodes: i32, graph: &Vec<Vec<i32>>) -> i32 {
        /// Return (max_height, max_dist)
        fn helper(node: i32, parent: i32, nodes: i32, graph: &Vec<Vec<i32>>) -> (i32, i32) {
            let mut subtree_heights = vec![];
            let mut max_dist = 0;
            for &child in graph[node as usize].iter() {
                if Solution::contain_node(nodes, child) && child != parent {
                    let (h, d) = helper(child, node, nodes, graph);
                    subtree_heights.push(h);
                    max_dist = max(max_dist, d);
                }
            }
            subtree_heights.sort_by(|h1, h2| h2.cmp(h1));
            if subtree_heights.is_empty() {
                // Leaf
                return (0, max_dist);
            }
            let len = subtree_heights.len();
            if len == 1 {
                // 1 subtree
                max_dist = max(max_dist, subtree_heights[0] + 1);
                return (subtree_heights[0] + 1, max_dist);
            }
            // 2 or more subtree
            max_dist = max(max_dist, subtree_heights[0] + subtree_heights[1] + 2);
            return (subtree_heights[0] + 1, max_dist);
        }
        let root = nodes.trailing_zeros() as i32 + 1;
        return helper(root, 0, nodes, graph).1;
    }
}

fn main() {
    {
        let n = 4;
        let edges = vec![vec![1, 2], vec![2, 3], vec![2, 4]];
        let ans = vec![3, 4, 0];
        assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), ans);
    }

    {
        let n = 2;
        let edges = vec![vec![1, 2]];
        let ans = vec![1];
        assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), ans);
    }

    {
        let n = 3;
        let edges = vec![vec![1, 2], vec![2, 3]];
        let ans = vec![2, 1];
        assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), ans);
    }
}
