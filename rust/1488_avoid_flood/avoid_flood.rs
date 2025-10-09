struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};
        let mut gmap: HashMap<i32, usize> = HashMap::new();
        let mut res = vec![-1; rains.len()];

        let mut que: BTreeSet<usize> = BTreeSet::new();
        for (idx, i) in rains.iter().enumerate() {
            if *i == 0 {
                que.insert(idx);
            } else {
                if let Some(x) = gmap.get(i) {
                    if que.is_empty() {
                        return vec![];
                    }
                    let tidx = que.range((x)..).next();
                    if tidx.is_none() {
                        return vec![];
                    }
                    let tidx = *tidx.unwrap();
                    que.remove(&tidx);
                    res[tidx] = *i;
                    gmap.insert(*i, idx);
                } else {
                    gmap.insert(*i, idx);
                }
            }
        }

        que.iter().for_each(|x| {
            res[*x] = 1;
        });

        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4], vec![-1, -1, -1, -1]),
        (vec![1, 2, 0, 0, 2, 1], vec![-1, -1, 2, 1, -1, -1]),
        (vec![1, 2, 0, 1, 2], vec![]),
    ];

    for (rains, ans) in tests {
        assert_eq!(Solution::avoid_flood(rains), ans);
    }
}
