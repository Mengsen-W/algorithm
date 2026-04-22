/*
 * @Date: 2022-03-20 02:00:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-20 02:13:08
 * @FilePath: /algorithm/2093_network_becomes_idle/network_becomes_idle.rs
 */

pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    let n = patience.len();
    let mut map = vec![vec![]; n];
    for e in edges {
        let (x, y) = (e[0] as usize, e[1] as usize);
        map[x].push(y);
        map[y].push(x);
    }
    let mut dis = vec![i32::MAX >> 1; n];
    let mut fifo = VecDeque::with_capacity(n);
    dis[0] = 0;
    fifo.push_back(0);
    while let Some(u) = fifo.pop_front() {
        for &v in map[u].iter() {
            if dis[v] > dis[u] + 1 {
                dis[v] = dis[u] + 1;
                fifo.push_back(v);
            }
        }
    }
    let mut ans = 0;
    for (d, p) in dis[1..].into_iter().zip(patience[1..].into_iter()) {
        let d = d << 1;
        let t = (d - 1) / p;
        ans = i32::max(ans, t * p + d);
    }
    ans as i32 + 1
}

fn main() {
    assert_eq!(
        network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
        8
    );
    assert_eq!(
        network_becomes_idle(vec![vec![0, 1], vec![0, 2], vec![1, 2]], vec![0, 10, 10]),
        3
    );
}
