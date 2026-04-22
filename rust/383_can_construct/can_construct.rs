/*
 * @Date: 2021-12-04 05:44:00
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-07
 */

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut hash = [0; 26];
        magazine
            .chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] += 1);
        ransom_note
            .chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] -= 1);
        !hash.iter().any(|&x| x < 0)
    }
}

fn main() {
    let tests = vec![("a", "b", false), ("aa", "ab", false), ("aa", "aab", true)];

    for (ransom_note, magazine, ans) in tests {
        assert_eq!(
            Solution::can_construct(ransom_note.to_string(), magazine.to_string()),
            ans
        );
    }
}
