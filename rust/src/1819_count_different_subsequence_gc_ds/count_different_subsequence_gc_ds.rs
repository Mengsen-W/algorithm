/*
 * @Date: 2023-01-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-14
 * @FilePath: /algorithm/1819_count_different_subsequence_gc_ds/count_different_subsequence_gc_ds.rs
 */

use std::cmp::{max, min};
use std::collections::HashSet;
fn gcd(a: i32, b: i32) -> i32 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

struct Solution;

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap();
        let mut occured = HashSet::new();
        for num in nums {
            occured.insert(num);
        }
        let mut ans = 0;
        for i in 1..=max_val {
            let mut sub_gcd = 0;
            for j in (i..=max_val).step_by(i as usize) {
                if occured.contains(&j) {
                    if sub_gcd == 0 {
                        sub_gcd = j;
                    } else {
                        sub_gcd = gcd(sub_gcd, j);
                    }
                    if sub_gcd == i {
                        ans += 1;
                        break;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    {
        let nums = vec![6, 10, 3];
        let ans = 5;
        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), ans);
    }

    {
        let nums = vec![5, 15, 40, 5, 6];
        let ans = 7;
        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), ans);
    }
}
