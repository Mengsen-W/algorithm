struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::{BTreeMap, HashMap};
        let mut groups: HashMap<i32, BTreeMap<i32, i32>> = HashMap::new();
        for &a in &nums {
            let mod_val = a % k;
            groups
                .entry(mod_val)
                .or_insert(BTreeMap::new())
                .entry(a)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut ans = 1;
        for g in groups.values() {
            let m = g.len();
            let mut f = vec![vec![0; 2]; m];
            let first_key = *g.keys().next().unwrap();
            f[0][0] = 1;
            f[0][1] = (1 << g[&first_key]) - 1;
            let mut i = 1;
            let mut prev_key = first_key;
            for (&curr_key, &count) in g.iter().skip(1) {
                f[i][0] = f[i - 1][0] + f[i - 1][1];
                if curr_key - prev_key == k {
                    f[i][1] = f[i - 1][0] * ((1 << count) - 1);
                } else {
                    f[i][1] = (f[i - 1][0] + f[i - 1][1]) * ((1 << count) - 1);
                }
                prev_key = curr_key;
                i += 1;
            }
            ans *= f[m - 1][0] + f[m - 1][1];
        }
        ans - 1
    }
}

fn main() {
    let tests = vec![(vec![2, 4, 6], 2, 4), (vec![1], 1, 1)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::beautiful_subsets(nums, k), ans);
    }
}
