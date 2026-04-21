/*
 * @Date: 2022-12-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-14
 * @FilePath: /algorithm/1697_distance_limited_paths_exist/distance_limited_paths_exist.rs
 */

pub fn distance_limited_paths_exist(
    n: i32,
    edge_list: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    fn find(fa: &mut Vec<i32>, x: i32) -> i32 {
        let x_index = x as usize;
        if fa[x_index] != x {
            fa[x_index] = find(fa, fa[x_index])
        }
        return fa[x_index];
    }

    fn merge(fa: &mut Vec<i32>, mut x: i32, mut y: i32) {
        x = find(fa, x);
        y = find(fa, y);
        fa[y as usize] = x;
    }

    let mut edge_list = edge_list;
    edge_list.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
    let mut index: Vec<usize> = (0..queries.len()).collect();
    index.sort_unstable_by(|&a, &b| queries[a][2].cmp(&queries[b][2]));
    let mut uf: Vec<i32> = (0..n).collect();
    let mut res = vec![false; queries.len()];

    let mut k = 0;
    for i in index {
        while k < edge_list.len() && edge_list[k][2] < queries[i][2] {
            merge(&mut uf, edge_list[k][0], edge_list[k][1]);
            k += 1;
        }
        res[i] = find(&mut uf, queries[i][0]) == find(&mut uf, queries[i][1]);
    }

    res
}

fn main() {
    {
        let n = 3;
        let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
        let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
        let ans = vec![false, true];
        assert_eq!(distance_limited_paths_exist(n, edge_list, queries), ans);
    }

    {
        let n = 5;
        let edge_list = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
        let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];
        let ans = vec![true, false];
        assert_eq!(distance_limited_paths_exist(n, edge_list, queries), ans);
    }
}
