/*
 * @Date: 2022-11-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-26
 * @FilePath: /algorithm/882_reachable_nodes/reachable_nodes.rs
 */

pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
    use std::cmp::min;
    use std::collections::{BinaryHeap, HashSet};
    let n = n as usize;
    let edges: Vec<(usize, usize, i32)> = edges
        .into_iter()
        .map(|arr| (arr[0] as usize, arr[1] as usize, arr[2]))
        .collect();

    let mut map: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for &(s, e, cnt) in edges.iter() {
        map[s].push((e, cnt));
        map[e].push((s, cnt));
    }

    let mut vis = HashSet::new();
    let mut heap = BinaryHeap::from([(max_moves, 0)]);
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; n];

    while let Some((moves, start)) = heap.pop() {
        if moves < 0 {
            break;
        }
        if vis.contains(&start) {
            continue;
        }
        vis.insert(start);
        for &(end, cnt) in map[start].iter() {
            if moves >= cnt + 1 && !vis.contains(&end) {
                heap.push((moves - cnt - 1, end));
            }
            res[start][end] = min(cnt, moves);
        }
    }

    let mut ans = vis.len() as i32;
    for (s, e, cnt) in edges.into_iter() {
        ans += min(cnt, res[s][e] + res[e][s]);
    }
    return ans;
}

fn main() {
    {
        let edges = vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]];
        let max_moves = 6;
        let n = 3;
        let ans = 13;
        assert_eq!(reachable_nodes(edges, max_moves, n), ans);
    }

    {
        let edges = vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]];
        let max_moves = 10;
        let n = 4;
        let ans = 23;
        assert_eq!(reachable_nodes(edges, max_moves, n), ans);
    }

    {
        let edges = vec![
            vec![1, 2, 4],
            vec![1, 4, 5],
            vec![1, 3, 1],
            vec![2, 3, 4],
            vec![3, 4, 5],
        ];
        let max_moves = 17;
        let n = 5;
        let ans = 1;
        assert_eq!(reachable_nodes(edges, max_moves, n), ans);
    }
}
