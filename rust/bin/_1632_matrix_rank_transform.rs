/*
 * @Date: 2023-01-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-25
 * @FilePath: /algorithm/rust/1632_matrix_rank_transform/matrix_rank_transform.rs
 */

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent: Vec<i32> = (0..n as i32).collect();
        UnionFind { parent }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] == x {
            x
        } else {
            self.find(self.parent[x as usize])
        }
    }

    fn union(&mut self, x: i32, y: i32) -> (i32, i32) {
        let px = self.find(x);
        let py = self.find(y);
        self.parent[px as usize] = py;
        (px, py)
    }
}

impl Solution {
    pub fn matrix_rank_transform(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let m = matrix.len();
        let n = matrix[0].len();
        let mut rank = vec![0; m + n];
        let mut inverse_map = HashMap::<i32, Vec<(i32, i32)>>::new();
        for i in 0..m {
            for j in 0..n {
                inverse_map
                    .entry(matrix[i][j])
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }
        let mut keys: Vec<i32> = inverse_map.keys().copied().collect();
        keys.sort_unstable();
        for key in keys {
            let mut uf = UnionFind::new(m + n);
            let mut rank2 = rank.to_vec();
            for &(r, c) in inverse_map.get(&key).unwrap() {
                let res = uf.union(r, c + m as i32);
                rank2[res.1 as usize] = std::cmp::max(rank2[res.1 as usize], rank2[res.0 as usize]);
            }
            for &(r, c) in inverse_map.get(&key).unwrap() {
                rank[r as usize] = rank2[uf.find(r) as usize] + 1;
                rank[c as usize + m] = rank[r as usize];
                matrix[r as usize][c as usize] = rank[r as usize];
            }
        }
        matrix
    }
}

struct Solution;

fn main() {
    {
        let matrix: Vec<Vec<i32>> = [[1, 2], [3, 4]].iter().map(|v| v.to_vec()).collect();
        let ans: Vec<Vec<i32>> = [[1, 2], [2, 3]].iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::matrix_rank_transform(matrix), ans);
    }

    {
        let matrix: Vec<Vec<i32>> = [[7, 7], [7, 7]].iter().map(|v| v.to_vec()).collect();
        let ans: Vec<Vec<i32>> = [[1, 1], [1, 1]].iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::matrix_rank_transform(matrix), ans);
    }

    {
        let matrix: Vec<Vec<i32>> = [[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = [[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(Solution::matrix_rank_transform(matrix), ans);
    }

    {
        let matrix: Vec<Vec<i32>> = [[7, 3, 6], [1, 4, 5], [9, 8, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = [[5, 1, 4], [1, 2, 3], [6, 3, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(Solution::matrix_rank_transform(matrix), ans);
    }
}
