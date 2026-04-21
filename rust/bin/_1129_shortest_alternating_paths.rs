/*
 * @Date: 2023-02-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-02
 * @FilePath: /algorithm/rust/1129_shortest_alternating_paths/shortest_alternating_paths.rs
 */

pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut red = vec![vec![]; n as usize];
    let mut blue = vec![vec![]; n as usize];
    for e in red_edges {
        red[e[0] as usize].push(e[1] as usize);
    }
    for e in blue_edges {
        blue[e[0] as usize].push(e[1] as usize);
    }
    let mut ans = vec![std::i32::MAX; n as usize];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0, 0));
    q.push_back((0, 1, 0));
    let mut visited = std::collections::HashSet::new();
    while let Some((i, c, d)) = q.pop_front() {
        if visited.contains(&(i, c)) {
            continue;
        }
        visited.insert((i, c));
        if ans[i] > d {
            ans[i] = d;
        }
        if c == 0 {
            for &j in &red[i] {
                q.push_back((j, 1, d + 1));
            }
        } else {
            for &j in &blue[i] {
                q.push_back((j, 0, d + 1));
            }
        }
    }
    ans.iter()
        .map(|&x| if x == std::i32::MAX { -1 } else { x as i32 })
        .collect()
}

fn main() {
    {
        let n = 3;
        let red_edges = [[0, 1], [1, 2]].iter().map(|s| s.to_vec()).collect();
        let blue_edges = Vec::new();
        let ans = vec![0, 1, -1];
        assert_eq!(shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }

    {
        let n = 3;
        let red_edges = [[0, 1]].iter().map(|s| s.to_vec()).collect();
        let blue_edges = [[2, 1]].iter().map(|s| s.to_vec()).collect();
        let ans = vec![0, 1, -1];
        assert_eq!(shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }

    {
        let n = 3;
        let red_edges = [[1, 0]].iter().map(|s| s.to_vec()).collect();
        let blue_edges = [[2, 1]].iter().map(|s| s.to_vec()).collect();
        let ans = vec![0, -1, -1];
        assert_eq!(shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }

    {
        let n = 3;
        let red_edges = [[0, 1]].iter().map(|s| s.to_vec()).collect();
        let blue_edges = [[1, 2]].iter().map(|s| s.to_vec()).collect();
        let ans = vec![0, 1, 2];
        assert_eq!(shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }

    {
        let n = 3;
        let red_edges = [[0, 1], [0, 2]].iter().map(|s| s.to_vec()).collect();
        let blue_edges = [[1, 0]].iter().map(|s| s.to_vec()).collect();
        let ans = vec![0, 1, 1];
        assert_eq!(shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }
}
