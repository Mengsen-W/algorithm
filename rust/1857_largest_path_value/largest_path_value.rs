struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let n = colors.len();
        // 邻接表
        let mut g = vec![vec![]; n];
        // 节点的入度统计，用于找出拓扑排序中最开始的节点
        let mut indeg = vec![0; n];
        for edge in edges {
            indeg[edge[1] as usize] += 1;
            g[edge[0] as usize].push(edge[1] as usize);
        }

        // 记录拓扑排序过程中遇到的节点个数
        // 如果最终 found 的值不为 n，说明图中存在环
        let mut found = 0;
        let mut f = vec![vec![0; 26]; n];
        let mut q = VecDeque::new();
        for i in 0..n {
            if indeg[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            found += 1;
            // 将节点 u 对应的颜色增加 1
            f[u][colors.as_bytes()[u] as usize - b'a' as usize] += 1;
            // 枚举 u 的后继节点 v
            for &v in &g[u] {
                indeg[v] -= 1;
                // 将 f(v,c) 更新为其与 f(u,c) 的较大值
                for c in 0..26 {
                    f[v][c] = f[v][c].max(f[u][c]);
                }
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if found != n {
            return -1;
        }
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(*f[i].iter().max().unwrap());
        }
        ans
    }
}

fn main() {
    let tests = vec![
        ("abaca", vec![[0, 1], [0, 2], [2, 3], [3, 4]], 3),
        ("a", vec![[0, 0]], -1),
    ];

    for (colors, edges, ans) in tests {
        assert_eq!(
            Solution::largest_path_value(
                colors.to_string(),
                edges.iter().map(|v| v.to_vec()).collect()
            ),
            ans
        );
    }
}
