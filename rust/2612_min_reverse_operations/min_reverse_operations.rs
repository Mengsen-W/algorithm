struct Solution;

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BTreeSet, HashSet, VecDeque};
        let n = n as usize;
        let p = p as usize;
        let k = k as usize;
        let ban: HashSet<i32> = banned.into_iter().collect();
        let mut sets = vec![BTreeSet::new(), BTreeSet::new()];
        for i in 0..n {
            if i != p && !ban.contains(&(i as i32)) {
                sets[i % 2].insert(i as i32);
            }
        }
        let mut ans = vec![-1; n];
        let mut q = VecDeque::new();
        q.push_back(p as i32);
        ans[p] = 0;
        while let Some(i) = q.pop_front() {
            let i = i as usize;
            let mn = (i as i32 - k as i32 + 1).max(k as i32 - i as i32 - 1) as usize;
            let mx = (i as i32 + k as i32 - 1).min(n as i32 * 2 - k as i32 - i as i32 - 1) as usize;
            let set = &mut sets[mx % 2];
            let mut to_remove = Vec::new();
            for &val in set.range(mn as i32..=mx as i32) {
                ans[val as usize] = ans[i] + 1;
                q.push_back(val);
                to_remove.push(val);
            }
            for val in to_remove {
                set.remove(&val);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (4, 0, vec![1, 2], 4, vec![0, -1, -1, 1]),
        (5, 0, vec![2, 4], 3, vec![0, -1, -1, -1, -1]),
        (4, 2, vec![0, 1, 3], 1, vec![-1, -1, 0, -1]),
    ];

    for (n, p, banned, k, ans) in tests {
        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), ans);
    }
}
