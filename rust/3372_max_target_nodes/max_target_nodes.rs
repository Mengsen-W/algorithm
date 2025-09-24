struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        fn dfs(node: usize, parent: i32, children: &Vec<Vec<i32>>, k: i32) -> i32 {
            if k < 0 {
                return 0;
            }
            let mut res = 1;
            for &child in &children[node] {
                if child == parent {
                    continue;
                }
                res += dfs(child as usize, node as i32, children, k - 1);
            }
            res
        }

        fn build(edges: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
            let n = edges.len() + 1;
            let mut children = vec![vec![]; n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                children[u].push(v as i32);
                children[v].push(u as i32);
            }
            let mut res = vec![0; n];
            for i in 0..n {
                res[i] = dfs(i, -1, &children, k);
            }
            res
        }

        let n = edges1.len() + 1;
        let count1 = build(edges1, k);
        let count2 = build(edges2, k - 1);
        let max_count2 = *count2.iter().max().unwrap();
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = count1[i] + max_count2;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 7],
                vec![1, 4],
                vec![4, 5],
                vec![4, 6],
            ],
            2,
            vec![9, 7, 9, 8, 8],
        ),
        (
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            1,
            vec![6, 3, 3, 3, 3],
        ),
    ];

    for (edges1, edges2, k, ans) in tests {
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }
}
