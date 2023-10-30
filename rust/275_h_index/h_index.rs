/*
 * @Date: 2021-07-12 08:06:47
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-30
 */

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n: i32 = citations.len() as i32;
        let (mut left, mut right): (i32, i32) = (0, n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            match citations[mid as usize] >= n - mid {
                true => right = mid - 1,
                false => left = mid + 1,
            }
        }
        n - left
    }
}

fn main() {
    let tests = vec![(vec![0, 1, 3, 5, 6], 3), (vec![1], 1)];

    for (citations, ans) in tests {
        assert_eq!(Solution::h_index(citations), ans);
    }
}
