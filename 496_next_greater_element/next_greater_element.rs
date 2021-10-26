/*
 * @Date: 2021-10-26 01:01:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-26 01:15:51
 * @FilePath: /algorithm/496_next_greater_element/next_greater_element.rs
 * @Description: file content
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut s: Vec<i32> = vec![0; nums2.len()];
        for i in (0..nums2.len()).rev() {
            while s.len() != 0 && s[s.len() - 1] <= nums2[i] {
                s.pop();
            }
            let t = if s.len() == 0 { -1 } else { s[s.len() - 1] };
            m.insert(nums2[i], t);
            s.push(nums2[i]);
        }
        nums1.iter().map(|x| *m.get(x).unwrap()).collect::<Vec<_>>()
    }
}

fn main() {
    {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let ans = vec![-1, 3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), ans);
    }
    {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let ans = vec![3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), ans);
    }
}
