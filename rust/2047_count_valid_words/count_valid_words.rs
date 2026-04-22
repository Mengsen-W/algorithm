/*
 * @Date: 2022-01-27 02:37:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-27 03:05:44
 */

struct Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut cnt = 0;
        for s in sentence.split(" ") {
            if Self::is_match(s) {
                cnt += 1;
            }
        }
        cnt
    }
    fn is_match(s: &str) -> bool {
        if s.len() == 0 {
            return false;
        }
        let (mut hyphen, n) = (0, s.len());
        let chars: Vec<char> = s.chars().collect();

        for i in 0..n {
            let ch = chars[i];

            if ch.is_numeric() {
                return false;
            }
            if ch.is_lowercase() {
                continue;
            }
            if ch == '-' {
                hyphen += 1;
                if hyphen > 1 {
                    return false;
                }
                if i as i32 - 1 < 0
                    || !chars[i - 1].is_lowercase()
                    || i + 1 >= n
                    || !chars[i + 1].is_lowercase()
                {
                    return false;
                }
            } else if i != n - 1 {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
    assert_eq!(
        Solution::count_valid_words("!this  1-s b8d!".to_string()),
        0
    );
    assert_eq!(
        Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
        5
    );
    assert_eq!(
        Solution::count_valid_words(
            "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.".to_string()
        ),
        6,
    );
}
