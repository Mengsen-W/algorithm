/*
 * @Date: 2021-11-08 00:08:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-08 00:27:38
 */

struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cnt_s = vec![0; 10];
        let mut cnt_g = vec![0; 10];
        let secret_len = secret.len();
        for i in 0..secret_len {
            if secret.as_bytes()[i] == guess.as_bytes()[i] {
                bulls += 1;
            } else {
                cnt_s[(secret.as_bytes()[i] - b'0') as usize] += 1;
                cnt_g[(guess.as_bytes()[i] - b'0') as usize] += 1;
            }
        }
        let mut cows = 0;
        for i in 0..10 {
            cows += std::cmp::min(cnt_s[i], cnt_g[i]);
        }
        bulls.to_string() + "A" + &cows.to_string() + "B"
    }
}

fn main() {
    assert_eq!(
        Solution::get_hint("1807".to_string(), "7810".to_string()),
        "1A3B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0111".to_string()),
        "1A1B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1".to_string(), "0".to_string()),
        "0A0B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1".to_string(), "1".to_string()),
        "1A0B".to_string()
    );
}
