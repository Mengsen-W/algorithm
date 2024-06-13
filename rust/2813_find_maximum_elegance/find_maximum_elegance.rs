struct Solution;

impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        use std::collections::{HashSet, VecDeque};
        items.sort_unstable_by_key(|item| -item[0]);
        let (mut category_set, mut st) = (HashSet::new(), VecDeque::new());
        let (mut res, mut profit) = (0 as i64, 0 as i64);
        for (i, item) in items.iter().enumerate() {
            if i < k as usize {
                profit += item[0] as i64;
                if !category_set.contains(&item[1]) {
                    category_set.insert(item[1]);
                } else {
                    st.push_back(item[0]);
                }
            } else if !category_set.contains(&item[1]) && !st.is_empty() {
                profit += (item[0] - st.back().unwrap()) as i64;
                st.pop_back();
                category_set.insert(item[1]);
            }
            res = res.max(profit + (category_set.len() * category_set.len()) as i64);
        }
        res as i64
    }
}

fn main() {
    let tests = vec![
        (vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2, 17),
        (vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]], 3, 19),
        (vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3, 7),
    ];

    for (items, k, ans) in tests {
        assert_eq!(Solution::find_maximum_elegance(items, k), ans);
    }
}
