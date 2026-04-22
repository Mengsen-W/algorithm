/*
 * @Date: 2021-09-21 09:05:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-21 09:39:13
 */

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.trim().len() == 0 {
            return 0;
        }
        s.split_whitespace().last().unwrap().len() as i32
    }
}





fn main() {
    {
        let s = "Hello World".to_string();
        assert_eq!(Solution::length_of_last_word(s), 5);
    }
    {
        let s = "   fly me   to   the moon  ".to_string();
        assert_eq!(Solution::length_of_last_word(s), 4);
    }
    {
        let s = "luffy is still joyboy".to_string();
        assert_eq!(Solution::length_of_last_word(s), 6);
    }
}
