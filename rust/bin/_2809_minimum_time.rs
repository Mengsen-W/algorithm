/*
 * @Date: 2024-01-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-20
 * @FilePath: /algorithm/rust/2809_minimum_time/minimum_time.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let mut pairs = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        pairs.sort_unstable_by(|&a, &b| a.1.cmp(&b.1));

        let n = pairs.len();
        let mut f = vec![0; n + 1];
        for (i, &(a, b)) in pairs.iter().enumerate() {
            for j in (1..=i + 1).rev() {
                f[j] = f[j].max(f[j - 1] + a + b * j as i32);
            }
        }

        let s1 = nums1.iter().sum::<i32>();
        let s2 = nums2.iter().sum::<i32>();
        for (t, &v) in f.iter().enumerate() {
            if s1 + s2 * t as i32 - v <= x {
                return t as i32;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], vec![1, 2, 3], 4, 3),
        (vec![1, 2, 3], vec![3, 3, 3], 4, -1),
    ];

    for (nums1, nums2, x, ans) in tests {
        assert_eq!(Solution::minimum_time(nums1, nums2, x), ans);
    }
}
