/*
 * @Date: 2024-02-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-27
 * @FilePath: /algorithm/rust/2867_count_paths/count_paths.rs
 */

struct Solution;

impl Solution {
    fn dfs(graph: &Vec<Vec<usize>>, nodes: &mut Vec<usize>, parent: usize, node: usize) -> i64 {
        nodes.push(node);

        let mut path = 1;
        for child in graph[node].iter() {
            if child != &parent {
                path += Solution::dfs(graph, nodes, node, *child);
            }
        }
        path
    }

    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize + 1];
        let mut is_prime = vec![true; n as usize + 1];
        let mut ways = vec![0_i64; n as usize + 1];
        let mut nodes: Vec<usize> = vec![];
        let (mut ans, mut sum) = (0, 1);

        // 筛法求质数
        is_prime[1] = false;
        for i in 2..=(n as f32).sqrt() as usize {
            if is_prime[i] {
                for j in (i.pow(2)..=n as usize).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        for edge in edges.into_iter() {
            if !is_prime[edge[1] as usize] {
                graph[edge[0] as usize].push(edge[1] as usize);
            }
            if !is_prime[edge[0] as usize] {
                graph[edge[1] as usize].push(edge[0] as usize);
            }
        }

        for i in 1..=n as usize {
            if is_prime[i] {
                sum = 1;
                for child in graph[i].iter() {
                    if ways[*child] == 0 {
                        nodes.clear();
                        let temp = Solution::dfs(&graph, &mut nodes, i, *child);
                        for node in nodes.iter() {
                            ways[*node] = temp;
                        }
                    }
                    ans += sum * ways[*child];
                    sum += ways[*child];
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]], 4),
        (
            6,
            vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]],
            6,
        ),
    ];

    for (n, edges, ans) in tests {
        assert_eq!(Solution::count_paths(n, edges), ans);
    }
}
