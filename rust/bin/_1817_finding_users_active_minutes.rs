/*
 * @Date: 2023-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-20
 * @FilePath: /algorithm/rust/1817_finding_users_active_minutes/finding_users_active_minutes.rs
 */

pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let k = k as usize;
    let mut ans = vec![0; k];
    let mut tab = HashMap::new();
    for log in logs {
        let t = tab.entry(log[0]).or_insert(HashSet::new());
        t.insert(log[1]);
    }
    for (_, t) in tab {
        ans[t.len() - 1] += 1;
    }
    ans
}

fn main() {
    {
        let logs = [[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let k = 5;
        let ans = [0, 2, 0, 0, 0].to_vec();
        assert_eq!(finding_users_active_minutes(logs, k), ans);
    }

    {
        let logs = [[1, 1], [2, 2], [2, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let k = 4;
        let ans = [1, 1, 0, 0].to_vec();
        assert_eq!(finding_users_active_minutes(logs, k), ans);
    }
}
