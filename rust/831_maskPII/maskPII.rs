/*
 * @Date: 2023-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-01
 * @FilePath: /algorithm/rust/831_maskPII/maskPII.rs
 */

pub fn mask_pii(s: String) -> String {
    if let Some(index) = s.find('@') {
        return s[0..1].to_lowercase() + "*****" + &s[(index - 1)..].to_lowercase();
    }

    let mut count = 0;
    let mut num_str = String::new();

    s.chars().rev().for_each(|c| {
        if c >= '0' && c <= '9' {
            count += 1;

            if count <= 4 {
                num_str.insert(0, c);
            }
        }
    });

    if count == 10 {
        "***-***-".to_string() + &num_str
    } else {
        "+".to_string() + &"*".repeat(count - 10) + &"-***-***-".to_string() + &num_str
    }
}

fn main() {
    {
        let s = "LeetCode@LeetCode.com".to_string();
        let ans = "l*****e@leetcode.com".to_string();
        assert_eq!(mask_pii(s), ans);
    }

    {
        let s = "AB@qq.com".to_string();
        let ans = "a*****b@qq.com".to_string();
        assert_eq!(mask_pii(s), ans);
    }

    {
        let s = "1(234)567-890".to_string();
        let ans = "***-***-7890".to_string();
        assert_eq!(mask_pii(s), ans);
    }

    {
        let s = "86-(10)12345678".to_string();
        let ans = "+**-***-***-5678".to_string();
        assert_eq!(mask_pii(s), ans);
    }
}
