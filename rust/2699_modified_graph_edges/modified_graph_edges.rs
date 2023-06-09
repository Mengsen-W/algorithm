/*
 * @Date: 2023-06-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-09
 * @FilePath: /algorithm/rust/2699_modified_graph_edges/modified_graph_edges.rs
 */

pub fn modified_graph_edges(
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    fn dijkstra(
        op: i32,
        source: usize,
        edges: &mut [Vec<i32>],
        adj_matrix: &mut Vec<Vec<i32>>,
        target: i32,
        from: &mut [i64],
    ) -> Vec<i64> {
        let n = adj_matrix.len();
        let (mut dist, mut used) = (vec![0x3f3f3f3f3f_i64; n], vec![false; n]);
        dist[source] = 0;
        (0..n - 1).for_each(|_| {
            let mut u = n;
            (0..n).for_each(|i| {
                if !used[i] && (u == n || dist[i] < dist[u]) {
                    u = i;
                }
            });
            used[u] = true;
            (0..n).for_each(|v| {
                if !used[v] && adj_matrix[u][v] != -1 {
                    let i = adj_matrix[u][v] as usize;
                    if edges[i][2] != -1 {
                        dist[v] = dist[v].min(dist[u] + (edges[i][2] as i64));
                    } else if op == 0 {
                        dist[v] = dist[v].min(dist[u] + 1);
                    } else {
                        let modify = target as i64 - dist[u] - from[v];
                        if modify > 0 {
                            dist[v] = dist[v].min(dist[u] + modify);
                            edges[i][2] = modify as i32;
                        } else {
                            edges[i][2] = target;
                        }
                    }
                }
            });
        });
        dist
    }
    let n = n as usize;
    let mut adj_matrix = vec![vec![-1; n]; n];
    let (source, dest, mut edges) = (source as usize, destination as usize, edges);
    edges.iter().enumerate().for_each(|(i, v)| {
        let (u, v) = (*v.first().unwrap() as usize, *v.get(1).unwrap() as usize);
        adj_matrix[u][v] = i as i32;
        adj_matrix[v][u] = i as i32;
    });
    let mut from = dijkstra(0, dest, &mut edges, &mut adj_matrix, target, &mut []);
    if from[source] > target as i64 {
        return vec![];
    }
    let from_source = dijkstra(1, source, &mut edges, &mut adj_matrix, target, &mut from);
    if from_source[dest] != target as i64 {
        return vec![];
    }
    edges
}

fn main() {
    {
        let n = 5;
        let edges = vec![
            vec![4, 1, -1],
            vec![2, 0, -1],
            vec![0, 3, -1],
            vec![4, 3, -1],
        ];
        let source = 0;
        let destination = 1;
        let target = 5;
        let ans = vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]];
        assert_eq!(
            modified_graph_edges(n, edges, source, destination, target),
            ans
        );
    }

    {
        let n = 5;
        let edges = vec![
            vec![4, 1, -1],
            vec![2, 0, -1],
            vec![0, 3, -1],
            vec![4, 3, -1],
        ];
        let source = 0;
        let destination = 1;
        let target = 5;
        let ans = vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]];
        assert_eq!(
            modified_graph_edges(n, edges, source, destination, target),
            ans
        );
    }

    {
        let n = 3;
        let edges = vec![vec![0, 1, -1], vec![0, 2, 5]];
        let source = 0;
        let destination = 2;
        let target = 6;
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(
            modified_graph_edges(n, edges, source, destination, target),
            ans
        );
    }

    {
        let n = 4;
        let edges = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]];
        let source = 0;
        let destination = 2;
        let target = 6;
        let ans = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, 1]];
        assert_eq!(
            modified_graph_edges(n, edges, source, destination, target),
            ans
        );
    }
}
