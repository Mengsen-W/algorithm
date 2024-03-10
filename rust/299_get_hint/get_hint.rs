/*
 * @Date: 2021-11-08 00:08:44
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-10
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
    let tests = vec![
        ("1807", "7810", "1A3B"),
        ("1123", "0111", "1A1B"),
        ("1", "0", "0A0B"),
        ("1", "1", "1A0B"),
    ];
    for (secret, guess, ans) in tests {
        assert_eq!(
            Solution::get_hint(secret.to_string(), guess.to_string()),
            ans.to_string()
        );
    }
}
