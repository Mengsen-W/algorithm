/*
 * @Date: 2021-08-05 14:47:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-05 15:46:36
 */

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        // Self::eventual_safe_nodes_dfs(graph)
        Self::eventual_safe_nodes_topology(graph)
    }

    fn eventual_safe_nodes_dfs(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn safe(graph: &Vec<Vec<i32>>, color: &mut Vec<i32>, x: usize) -> bool {
            if color[x] > 0 {
                return color[x] == 2;
            }
            color[x] = 1;
            if graph[x].iter().any(|&y| !safe(graph, color, y as usize)) {
                return false;
            }
            color[x] = 2;
            true
        }
        let mut color = vec![0; graph.len()];
        (0..graph.len())
            .into_iter()
            .filter(|&i| safe(&graph, &mut color, i))
            .map(|i| i as i32)
            .collect()
    }
    fn eventual_safe_nodes_topology(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut rg: Vec<Vec<i32>> = vec![vec![]; n];
        let mut in_deg: Vec<i32> = vec![0; n];
        for x in 0..n {
            for &y in &graph[x] {
                rg[y as usize].push(x as i32);
            }
            in_deg[x] = graph[x].len() as i32;
        }
        let mut q: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
        for i in 0..n {
            if in_deg[i] == 0 {
                q.push_back(i as i32);
            }
        }

        while !q.is_empty() {
            let y = *q.front().unwrap();
            q.pop_front();
            for &x in &rg[y as usize] {
                in_deg[x as usize] -= 1;
                if in_deg[x as usize] == 0 {
                    q.push_back(x);
                }
            }
        }
        let mut ans = Vec::new();
        for i in 0..n {
            if in_deg[i] == 0 {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() {
    {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let ans = vec![2, 4, 5, 6];
        assert_eq!(Solution::eventual_safe_nodes(graph), ans);
    }
    {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let ans = vec![4];
        assert_eq!(Solution::eventual_safe_nodes(graph), ans);
    }
}
