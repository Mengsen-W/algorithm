/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/rust/2698_punishment_number/punishment_number.rs
 */

struct Solution;

impl Solution {
    pub fn sub_punishment_number(n: i32) -> i32 {
        use std::{cmp::Ordering, collections::VecDeque};
        let ds = (n * n).to_string();
        let n_ds = ds.len();
        let mut stack = VecDeque::from([(0, 0)]);
        while let Some((start, acc)) = stack.pop_back() {
            for end in start + 1..=n_ds {
                let acc = acc + (&ds[start..end]).parse::<i32>().unwrap();
                match acc.cmp(&n) {
                    Ordering::Equal if end == n_ds => return n * n,
                    Ordering::Equal | Ordering::Less => stack.push_back((end, acc)),
                    _ => (),
                }
            }
        }
        0
    }
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n).map(Self::sub_punishment_number).sum()
    }
}

fn main() {
    let tests = vec![(10, 182), (37, 1478)];

    for (n, ans) in tests {
        assert_eq!(Solution::punishment_number(n), ans);
    }
}
