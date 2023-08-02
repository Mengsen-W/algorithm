/*
 * @Date: 2023-08-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-02
 * @FilePath: /algorithm/rust/822_flipgame/flipgame.rs
 */

struct Solution;
impl Solution {
    fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = fronts.len();
        let mut s = HashSet::new();
        let mut ans = std::i32::MAX;
        for i in 0..n {
            if fronts[i] == backs[i] {
                s.insert(fronts[i]);
            }
        }
        for i in 0..n {
            if !s.contains(&fronts[i]) {
                ans = ans.min(fronts[i]);
            }
            if !s.contains(&backs[i]) {
                ans = ans.min(backs[i]);
            }
        }
        if ans == std::i32::MAX {
            0
        } else {
            ans
        }
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3], 2)];
    for (fronts, backs, ans) in tests {
        assert_eq!(Solution::flipgame(fronts, backs), ans)
    }
}
