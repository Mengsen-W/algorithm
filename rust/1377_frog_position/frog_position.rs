/*
 * @Date: 2023-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-24
 * @FilePath: /algorithm/rust/1377_frog_position/frog_position.rs
 */

pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    use std::collections::VecDeque;
    let mut g = vec![vec![]; n as usize + 1];
    for v in edges {
        g[v[0] as usize].push(v[1] as usize);
        g[v[1] as usize].push(v[0] as usize);
    }
    let mut q = VecDeque::from([(1, 1f64)]); // (当前点，概率)
    let mut vis = vec![false; n as usize + 1];
    vis[1] = true;
    for i in 0..=t {
        let mut tq = VecDeque::new();
        while let Some((u, k)) = q.pop_front() {
            // 下一步可以访问的结点数
            let x = g[u].iter().filter(|&&i| !vis[i as usize]).count();
            // 当前点等于target且不能继续再跳了，直接返回结果
            if u == target as usize && (x == 0 || i == t) {
                return k;
            }
            for &v in g[u].iter() {
                if vis[v] {
                    continue;
                }
                vis[v] = true;
                tq.push_back((v, k / x as f64));
            }
        }
        q = tq;
    }
    // target的深度大于t
    0.0
}

fn main() {
    {
        let n = 7;
        let edges = [[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let t = 2;
        let target = 4;
        let ans = 0.16666666666666666;
        assert_eq!(frog_position(n, edges, t, target), ans);
    }

    {
        let n = 7;
        let edges = [[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let t = 1;
        let target = 7;
        let ans = 0.3333333333333333;
        assert_eq!(frog_position(n, edges, t, target), ans);
    }
}
