/*
 * @Date: 2023-05-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-03
 * @FilePath: /algorithm/rust/1003_is_valid/is_valid.rs
 */

pub fn is_valid(s: String) -> bool {
    let mut stk = Vec::<char>::new();
    for c in s.chars() {
        match c {
            'a'..='b' => stk.push(c),
            'c' => {
                let len = stk.len();
                if len < 2 || stk[len - 1] != 'b' || stk[len - 2] != 'a' {
                    return false;
                }
                stk.pop();
                stk.pop();
            }
            _ => (),
        }
    }
    stk.len() == 0
}

fn main() {
    {
        let s = "aabcbc".to_string();
        let ans = true;
        assert_eq!(is_valid(s), ans);
    }

    {
        let s = "abcabcababcc".to_string();
        let ans = true;
        assert_eq!(is_valid(s), ans);
    }

    {
        let s = "abccba".to_string();
        let ans = false;
        assert_eq!(is_valid(s), ans);
    }
}
