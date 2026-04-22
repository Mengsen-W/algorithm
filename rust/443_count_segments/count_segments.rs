/*
 * @Date: 2021-10-07 18:46:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-07 18:48:03
 * @FilePath: /algorithm/443_count_segments/count_segments.rs
 * @Description: file content
 */

struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::count_segments("Hello, my name is John".to_string()),
        5
    );
}
