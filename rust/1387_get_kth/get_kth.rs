struct Solution;

use std::collections::HashMap;

impl Solution {
    fn get_f(x: i32, f: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&val) = f.get(&x) {
            return val;
        }
        let res = if x == 1 {
            0
        } else if x & 1 == 1 {
            Self::get_f(x * 3 + 1, f) + 1
        } else {
            Self::get_f(x / 2, f) + 1
        };
        f.insert(x, res);
        res
    }

    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut f: HashMap<i32, i32> = HashMap::new();
        let mut v: Vec<i32> = (lo..=hi).collect();
        v.sort_by(|&u, &v| {
            let f1 = Self::get_f(u, &mut f);
            let f2 = Self::get_f(v, &mut f);
            if f1 != f2 {
                f1.cmp(&f2)
            } else {
                u.cmp(&v)
            }
        });
        v[k as usize - 1]
    }
}

fn main() {
    let tests = vec![(12, 15, 2, 13), (7, 11, 4, 7)];

    for (lo, hi, k, ans) in tests {
        assert_eq!(Solution::get_kth(lo, hi, k), ans);
    }
}
