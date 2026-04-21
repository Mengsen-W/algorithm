/*
 * @Date: 2021-10-15 09:16:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-15 10:18:31
 */

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut prev = String::from("1");
        for _ in 2..=n {
            let mut curr = String::new();
            let mut start = 0;
            let mut pos = 0;
            let size = prev.len();
            while pos < size {
                while (pos < size) && prev.chars().nth(pos) == prev.chars().nth(start) {
                    pos += 1;
                }
                curr +=
                    &((pos - start).to_string() + &prev.chars().nth(start).unwrap().to_string());
                start = pos;
            }
            prev = curr;
        }
        prev
    }
}

fn main() {
    assert_eq!(Solution::count_and_say(1), "1");
    assert_eq!(Solution::count_and_say(4), "1211");
}
