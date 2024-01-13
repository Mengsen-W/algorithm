/*
 * @Date: 2024-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-13
 * @FilePath: /algorithm/rust/2182_repeat_limited_string/repeat_limited_string.rs
 */

struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        const N: usize = 26;
        let mut count: Vec<i32> = vec![0; N];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut ret = String::new();
        let mut m = 0;
        let mut i = N as i32 - 1;
        let mut j = N as i32 - 2;
        while i >= 0 && j >= 0 {
            if count[i as usize] == 0 {
                m = 0;
                i -= 1;
            } else if m < repeat_limit {
                count[i as usize] -= 1;
                ret.push(('a' as u8 + i as u8) as char);
                m += 1;
            } else if j >= i || count[j as usize] == 0 {
                j -= 1;
            } else {
                count[j as usize] -= 1;
                ret.push(('a' as u8 + j as u8) as char);
                m = 0;
            }
        }
        ret
    }
}

fn main() {
    let tests = vec![("cczazcc", 3, "zzcccac"), ("aababab", 2, "bbabaa")];

    for (s, repeat_limit, ans) in tests {
        assert_eq!(
            Solution::repeat_limited_string(s.to_string(), repeat_limit),
            ans
        );
    }
}
