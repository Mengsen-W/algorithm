/*
 * @Date: 2021-07-12 08:24:46
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-29
 */

struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let len = citations.len();
        for x in citations.iter().enumerate().map(|(i, value)| {
            if *value as usize >= len - i {
                return len - i;
            }
            return 99999999;
        }) {
            if x != 99999999 {
                return x as i32;
            }
        }
        return 0;
    }
}

fn main() {
    let tests = vec![
        (vec![3, 0, 6, 1, 5], 3),
        (vec![1, 3, 1], 1),
        (vec![0, 1, 3, 5, 6], 3),
    ];

    for (citations, ans) in tests {
        assert_eq!(Solution::h_index(citations), ans);
    }
}
