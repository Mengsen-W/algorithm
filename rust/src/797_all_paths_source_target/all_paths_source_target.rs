/*
 * @Date: 2021-08-25 10:42:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-25 11:37:24
 */

struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut stk: Vec<i32> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        stk.push(0);
        fn dfs(graph: &Vec<Vec<i32>>, stk: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, x: i32, n: i32) {
            if x == n {
                ans.push(stk.clone());
                return;
            }
            for y in graph[x as usize].clone() {
                stk.push(y);
                dfs(graph, stk, ans, y, n);
                stk.pop();
            }
        }
        dfs(&graph, &mut stk, &mut ans, 0, graph.len() as i32 - 1);
        ans
    }
}

fn main() {
    {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let ans = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(Solution::all_paths_source_target(graph), ans);
    }
    {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let ans = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(Solution::all_paths_source_target(graph), ans);
    }
    {
        let graph = vec![vec![1], vec![]];
        let ans = vec![vec![0, 1]];
        assert_eq!(Solution::all_paths_source_target(graph), ans);
    }
    {
        let graph = vec![vec![1, 2, 3], vec![2], vec![3], vec![]];
        let ans = vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]];
        assert_eq!(Solution::all_paths_source_target(graph), ans);
    }
    {
        let graph = vec![vec![1, 3], vec![2], vec![3], vec![]];
        let ans = vec![vec![0, 1, 2, 3], vec![0, 3]];
        assert_eq!(Solution::all_paths_source_target(graph), ans);
    }
}
