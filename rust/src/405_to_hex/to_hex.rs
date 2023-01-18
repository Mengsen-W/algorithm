/*
 * @Date: 2021-10-02 08:34:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-02 09:05:13
 */

struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        let mut num = num as u32;
        let mut ans = String::new();
        let s = "0123456789abcdef".chars().collect::<Vec<_>>();
        while num != 0 {
            ans = s[(num & 0xf) as usize].to_string() + &ans;
            num >>= 4;
        }
        ans
    }
}

fn main() {
    assert_eq!(Solution::to_hex(26), "1a");
    assert_eq!(Solution::to_hex(-1), "ffffffff");
    assert_eq!(Solution::to_hex(16), "10");
}
