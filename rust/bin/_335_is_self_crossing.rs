/*
 * @Date: 2021-10-29 02:27:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-29 02:45:01
 */

struct Solution;

impl Solution {
    pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
        let n = distance.len();
        let mut i = 0;
        while i < n && (i < 2 || distance[i] > distance[i - 2]) {
            i += 1;
        }
        if i == n {
            return false;
        }
        if (i == 3 && distance[i] == distance[i - 2])
            || (i >= 4 && distance[i] >= distance[i - 2] - distance[i - 4])
        {
            distance[i - 1] -= distance[i - 3];
        }
        i += 1;
        while i < n && distance[i] < distance[i - 2] {
            i += 1;
        }
        return i != n;
    }
}

fn main() {
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
    assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);
}
