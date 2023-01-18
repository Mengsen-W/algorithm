/*
 * @Date: 2022-01-24 09:19:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-24 09:49:50
 */

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let n = n as usize;
    let mut graph = vec![vec![]; n + 1];

    for edge in edges {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }

    let mut path = vec![vec![i32::MAX; 2]; n + 1];
    path[1][0] = 0;
    let mut q: std::collections::VecDeque<(i32, i32)> = std::collections::VecDeque::new();
    q.push_back((1, 0));
    while path[n][1] == i32::MAX {
        if let Some((cur, len)) = q.pop_front() {
            for next in &graph[cur as usize] {
                let next_usize = *next as usize;
                if len + 1 < path[next_usize][0] {
                    path[next_usize][0] = len + 1;
                    q.push_back((*next, len + 1));
                } else if len + 1 > path[next_usize][0] && len + 1 < path[next_usize][1] {
                    path[next_usize][1] = len + 1;
                    q.push_back((*next, len + 1));
                }
            }
        }
    }

    let mut ret = 0;
    for _ in 0..path[n][1] {
        if ret % (2 * change) >= change {
            ret = ret + (2 * change - ret % (2 * change));
        }
        ret += time;
    }
    ret
}

fn main() {
    assert_eq!(
        second_minimum(
            5,
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
            3,
            5
        ),
        13
    );

    assert_eq!(second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
}
