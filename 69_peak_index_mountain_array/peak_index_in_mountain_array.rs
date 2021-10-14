/*
 * @Date: 2021-10-14 08:47:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-14 09:04:23
 */

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut l, mut r) = (1, arr.len() - 2);
        while l < r {
            let m = (l + r) >> 1;
            if arr[m] < arr[m + 1] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![1, 3, 5, 4, 2]),
        2
    );
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
        2
    );
}
