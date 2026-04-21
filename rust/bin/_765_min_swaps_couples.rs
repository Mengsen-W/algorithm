/*
 * @Date: 2023-11-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-11
 * @FilePath: /algorithm/rust/765_min_swaps_couples/min_swaps_couples.rs
 */

struct Solution;
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut parent = (0..60).collect::<Vec<usize>>();
        for i in 0..30 {
            parent[i * 2 + 1] = parent[i * 2];
        }
        fn union(parent: &mut Vec<usize>, idx1: usize, idx2: usize) {
            let idx1 = find(parent, idx1);
            let idx2 = find(parent, idx2);
            parent[idx1] = idx2;
        }
        fn find(parent: &mut Vec<usize>, mut idx: usize) -> usize {
            while parent[idx] != idx {
                parent[idx] = parent[parent[idx]];
                idx = parent[idx];
            }
            idx
        }
        let mut ans = row.len() / 2;
        for i in 0..row.len() / 2 {
            if find(&mut parent, row[2 * i] as usize) != find(&mut parent, row[2 * i + 1] as usize)
            {
                union(&mut parent, row[2 * i] as usize, row[2 * i + 1] as usize);
                ans -= 1;
            }
        }
        (row.len() / 2 - ans) as i32
    }
}

fn main() {
    let tests = vec![(vec![0, 2, 1, 3], 1), (vec![3, 2, 0, 1], 0)];

    for (row, ans) in tests {
        assert_eq!(Solution::min_swaps_couples(row), ans);
    }
}
