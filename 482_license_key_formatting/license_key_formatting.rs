/*
 * @Date: 2021-10-04 17:48:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-04 18:00:36
 */

struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        s.to_ascii_uppercase()
            .chars()
            .filter(|&c| c != '-')
            .rev()
            .fold((String::new(), -1), |(mut v, mut step), si| {
                if step == -1 {
                    step = 0;
                } else if step % k == 0 {
                    v.push('-');
                }
                step += 1;
                v.push(si);
                (v, step)
            })
            .0
            .chars()
            .rev()
            .collect::<String>()
    }
}

fn main() {
    {
        let s = "5F3Z-2e-9-w".to_string();
        let k = 4;
        let ans = "5F3Z-2E9W".to_string();
        assert_eq!(Solution::license_key_formatting(s, k), ans);
    }
    {
        let s = "2-5g-3-J".to_string();
        let k = 2;
        let ans = "2-5G-3J".to_string();
        assert_eq!(Solution::license_key_formatting(s, k), ans);
    }
}
