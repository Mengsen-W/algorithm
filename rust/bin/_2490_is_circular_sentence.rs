/*
 * @Date: 2023-06-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-30
 * @FilePath: /algorithm/rust/2490_is_circular_sentence/is_circular_sentence.rs
 */

pub fn is_circular_sentence(s: String) -> bool {
    let n = s.len();
    //字符串要访问某个个位置对应的字符，需要转化成字符数组然后使用nth(i)方法来访问第i个位置对应的字符
    if s.chars().nth(0) != s.chars().nth(n - 1) {
        return false;
    }

    let s: Vec<char> = s.chars().collect();
    for i in 0..n {
        if s[i] == ' ' {
            if s[i + 1] == ' ' {
                return false;
            } else if s[i - 1] != s[i + 1] {
                return false;
            }
        }
    }

    true
}

fn main() {
    let test_map = vec![
        ("leetcode exercises sound delightful", true),
        ("eetcode", true),
        ("Leetcode is cool", false),
    ];

    for item in test_map {
        assert_eq!(is_circular_sentence(item.0.to_string()), item.1);
    }
}
