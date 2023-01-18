/*
 * @Date: 2021-12-15 06:58:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-15 08:08:28
 */

pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = quiet.len();
    let mut g: Vec<Vec<i32>> = vec![vec![]; n];
    let mut in_deg: Vec<i32> = vec![0; n];
    for a in richer {
        g[a[0] as usize].push(a[1]);
        in_deg[a[1] as usize] += 1;
    }

    let mut ans: Vec<i32> = (0..n).map(|i| i as i32).collect();
    let mut q: VecDeque<i32> = in_deg
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x == 0)
        .map(|(i, _)| i as i32)
        .collect();
    while let Some(x) = q.pop_front() {
        for y in &g[x as usize] {
            if quiet[ans[x as usize] as usize] < quiet[ans[*y as usize] as usize] {
                ans[*y as usize] = ans[x as usize];
            }
            in_deg[*y as usize] -= 1;
            if in_deg[*y as usize] == 0 {
                q.push_back(*y);
            }
        }
    }
    ans
}

fn main() {
    {
        let richer = vec![
            vec![1, 0],
            vec![2, 1],
            vec![3, 1],
            vec![3, 7],
            vec![4, 3],
            vec![5, 3],
            vec![6, 3],
        ];
        let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
        let ans = vec![5, 5, 2, 5, 4, 5, 6, 7];
        assert_eq!(loud_and_rich(richer, quiet), ans);
    }

    {
        let richer = vec![];
        let quiet = vec![0];
        let ans = vec![0];
        assert_eq!(loud_and_rich(richer, quiet), ans);
    }
}
