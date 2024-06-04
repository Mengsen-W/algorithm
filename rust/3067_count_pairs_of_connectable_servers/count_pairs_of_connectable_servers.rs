struct Solution;

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n];
        for edge in edges.iter() {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let cost = edge[2];
            graph[x].push((y, cost));
            graph[y].push((x, cost));
        }

        fn dfs(
            graph: &Vec<Vec<(usize, i32)>>,
            p: usize,
            root: usize,
            curr: i32,
            signal_speed: i32,
        ) -> i32 {
            let mut res = 0;
            if curr == 0 {
                res += 1;
            }
            for &(v, cost) in &graph[p] {
                if v != root {
                    res += dfs(graph, v, p, (curr + cost) % signal_speed, signal_speed);
                }
            }
            res
        }

        let mut ans = vec![0; n];
        for i in 0..n {
            let mut pre = 0;
            for &(v, cost) in &graph[i] {
                let cnt = dfs(&graph, v, i, cost % signal_speed, signal_speed);
                ans[i] += pre * cnt;
                pre += cnt;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 1, 1],
                vec![1, 2, 5],
                vec![2, 3, 13],
                vec![3, 4, 9],
                vec![4, 5, 2],
            ],
            1,
            vec![0, 4, 6, 6, 4, 0],
        ),
        (
            vec![
                vec![0, 6, 3],
                vec![6, 5, 3],
                vec![0, 3, 1],
                vec![3, 2, 7],
                vec![3, 1, 6],
                vec![3, 4, 2],
            ],
            3,
            vec![2, 0, 0, 0, 0, 0, 2],
        ),
    ];

    for (edges, signal_speed, ans) in tests {
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(edges, signal_speed),
            ans
        );
    }
}
