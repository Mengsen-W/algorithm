/*
 * @Date: 2023-05-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-28
 * @FilePath: /algorithm/rust/1439_kth_smallest/kth_smallest.rs
 */

pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::collections::{BinaryHeap, HashSet};
    let mut h = BinaryHeap::new();
    let mut s = HashSet::new();
    let n = mat.len();
    let m = mat[0].len() - 1;

    h.push((-mat.iter().map(|r| r[0]).sum::<i32>(), vec![0; n]));

    for _ in 1..k {
        let (c, r) = h.pop().unwrap();

        for i in 0..n {
            if r[i] < m {
                let mut tmp = r.clone();
                tmp[i] += 1;

                if !s.contains(&tmp) {
                    s.insert(tmp.clone());
                    h.push((c + mat[i][tmp[i] - 1] - mat[i][tmp[i]], tmp));
                }
            }
        }
    }

    -h.pop().unwrap().0
}

fn main() {
    {
        let mat = vec![vec![1, 3, 11], vec![2, 4, 6]];
        let k = 5;
        let ans = 7;
        assert_eq!(kth_smallest(mat, k), ans);
    }

    {
        let mat = vec![vec![1, 3, 11], vec![2, 4, 6]];
        let k = 9;
        let ans = 17;
        assert_eq!(kth_smallest(mat, k), ans);
    }

    {
        let mat = vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]];
        let k = 7;
        let ans = 9;
        assert_eq!(kth_smallest(mat, k), ans);
    }

    {
        let mat = vec![vec![1, 1, 10], vec![2, 2, 9]];
        let k = 7;
        let ans = 12;
        assert_eq!(kth_smallest(mat, k), ans);
    }
}
