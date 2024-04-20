/*
 * @Date: 2024-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-20
 * @FilePath: /algorithm/rust/39_combination_sum/combination_sum.rs
 */

struct Solution;

impl Solution {
    fn dfs(i: usize, s: i32, candidates: &Vec<i32>, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if s == 0 {
            ans.push(t.clone());
            return;
        }
        if s < candidates[i] {
            return;
        }
        for j in i..candidates.len() {
            t.push(candidates[j]);
            Self::dfs(j, s - candidates[j], candidates, t, ans);
            t.pop();
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::new();
        Self::dfs(0, target, &candidates, &mut vec![], &mut ans);
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]),
        (
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        ),
        (vec![2], 1, vec![]),
    ];

    for (candidates, target, ans) in tests {
        assert_eq!(Solution::combination_sum(candidates, target), ans);
    }
}
