/*
 * @Date: 2023-09-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-05
 * @FilePath: /algorithm/rust/2605_min_number/min_number.rs
 */

struct Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums1.iter().fold(100, |ans, &n1| {
            nums2.iter().fold(ans, |ans, &n2| {
                ans.min(if n1 == n2 {
                    n1
                } else {
                    n1.min(n2) * 10 + n1.max(n2)
                })
            })
        })
    }
}

fn main() {
    let tests = vec![
        (vec![4, 1, 3], vec![5, 6], 15),
        (vec![3, 5, 2, 6], vec![3, 1, 7], 3),
    ];

    for (nums1, nums2, ans) in tests {
        assert_eq!(Solution::min_number(nums1, nums2), ans);
    }
}
