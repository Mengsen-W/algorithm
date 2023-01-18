/*
 * @Date: 2021-09-07 16:57:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:25:25
 */

struct Solution;

impl Solution {
    pub fn balanced_string_split(mut s: String) -> i32 {
        let mut count = 0;
        s.drain(..)
            .filter(|c| {
                match c {
                    'L' => count += 1,
                    'R' => count -= 1,
                    _ => (),
                }
                count == 0
            })
            .count() as i32
    }
}

fn main() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
}
