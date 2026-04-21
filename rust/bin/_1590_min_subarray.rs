struct Solution;

impl Solution {
    pub fn min_subarray(muns: Vec<i32>, p: i32) -> i32 {
        use std::collections::HashMap;
        let x = (muns.iter().map(|&x| x as i64).sum::<i64>() % p as i64) as i32;
        if x == 0 {
            return 0;
        }
        let len = muns.len();
        let mut idx = HashMap::with_capacity(len);
        idx.insert(0, -1);
        let len = len as i32;
        let mut ans = len;
        let mut y = 0;
        for (i, &v) in muns.iter().enumerate() {
            let i = i as i32;
            y = (y + v) % p;
            if let Some(t) = idx.get(&((y - x + p) % p)) {
                ans = ans.min(i - *t);
            }
            *idx.entry(y).or_insert(0) = i;
        }
        if ans == len {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    let tests = vec![
        (vec![3, 1, 4, 2], 6, 1),
        (vec![6, 3, 5, 2], 9, 2),
        (vec![1, 2, 3], 3, 0),
        (vec![1, 2, 3], 7, -1),
        (vec![1000000000, 1000000000, 1000000000], 3, 0),
    ];

    for (nums, p, expect) in tests {
        assert_eq!(Solution::min_subarray(nums, p), expect);
    }
}
