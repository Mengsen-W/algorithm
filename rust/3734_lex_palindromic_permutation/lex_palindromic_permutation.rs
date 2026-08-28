struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let n = s.len();
        // 特殊情况：长度为1
        if n == 1 {
            return if s > target { s } else { String::new() };
        }

        // 统计每个字符的出现次数
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        // 检查是否能构成回文串，并记录奇数个的字符
        let mut odd_char = String::new();
        for i in 0..26 {
            if cnt[i] % 2 == 1 {
                // 超过一个字符出现奇数次，无法构成回文
                if !odd_char.is_empty() {
                    return String::new();
                }
                odd_char = ((b'a' + i as u8) as char).to_string();
            }
            cnt[i] /= 2; // 只需要一半的字符来构造左半部分
        }

        let mut prefix = String::new();

        // 贪心构造左半部分的每一位
        for i in 0..n / 2 {
            let mut found = false;
            // 尝试放置字典序最小的字符
            for j in 0..26 {
                if cnt[j] == 0 {
                    continue;
                }

                cnt[j] -= 1;

                // 检查函数
                let mut left = prefix.clone();
                left.push((b'a' + j as u8) as char);
                for k in (0..26).rev() {
                    for _ in 0..cnt[k] {
                        left.push((b'a' + k as u8) as char);
                    }
                }

                let mut palindrome = left.clone();
                palindrome.push_str(&odd_char);
                let reversed_left: String = left.chars().rev().collect();
                palindrome.push_str(&reversed_left);

                if palindrome > target {
                    // 如果构造的回文串大于target，则选择该字符
                    prefix.push((b'a' + j as u8) as char);
                    found = true;
                    break;
                } else {
                    cnt[j] += 1; // 不满足条件，恢复计数
                }
            }
            if !found {
                return String::new(); // 无法构造出大于target的回文串
            }

            if prefix.as_bytes()[i] > target.as_bytes()[i] {
                // prefix已经大于target
                let mut left = prefix.clone();
                for j in 0..26 {
                    for _ in 0..cnt[j] {
                        left.push((b'a' + j as u8) as char);
                    }
                }
                let mut palindrome = left.clone();
                palindrome.push_str(&odd_char);
                let reversed_left: String = left.chars().rev().collect();
                palindrome.push_str(&reversed_left);
                return palindrome;
            }
        }

        // 构造最终的回文串
        let mut ans = prefix.clone();
        ans.push_str(&odd_char);
        let reversed_prefix: String = prefix.chars().rev().collect();
        ans.push_str(&reversed_prefix);
        ans
    }
}

fn main() {
    let tests = vec![
        ("baba", "abba", "baab"),
        ("baba", "bbaa", ""),
        ("abc", "abb", ""),
        ("aac", "abb", "aca"),
    ];

    for (s, target, expected) in tests {
        assert_eq!(
            Solution::lex_palindromic_permutation(s.to_string(), target.to_string()),
            expected
        );
    }
}
