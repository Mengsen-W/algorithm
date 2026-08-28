struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut cnt = vec![0i32; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut res = String::new();
        // let n = target.len();

        for (i, c) in target.chars().enumerate() {
            let target_char = (c as u8 - b'a') as usize;

            // 情况1：先尝试在当前位置放置与 target[i] 相同的字符
            if cnt[target_char] > 0 {
                cnt[target_char] -= 1;
                // 检查剩余字符能否构成大于 target[i+1:] 的字符串
                if Self::can_form_greater(&cnt, &target[i + 1..]) {
                    res.push(c);
                    continue;
                }
                // 不能构成更大的字符串，回溯
                cnt[target_char] += 1;
            }

            // 情况2：在当前位置放置一个大于 target[i] 的字符
            for j in (target_char + 1)..26 {
                if cnt[j] > 0 {
                    cnt[j] -= 1;
                    res.push((b'a' + j as u8) as char);
                    // 剩余位置按最小字典序填充
                    res.push_str(&Self::get_min_string(&cnt));
                    return res;
                }
            }

            // 无法找到可行方案, 直接返回
            return String::new();
        }

        String::new()
    }

    // 检查剩余字符是否能构成大于 suffix 的字符串
    fn can_form_greater(cnt: &[i32], suffix: &str) -> bool {
        let max_str = Self::get_max_string(cnt);
        max_str.as_str() > suffix
    }

    // 获取最小字典序字符串（升序排列）
    fn get_min_string(cnt: &[i32]) -> String {
        let total_len: usize = cnt.iter().map(|&c| c as usize).sum();
        let mut res = String::with_capacity(total_len);

        for i in 0..26 {
            res.push_str(
                &((b'a' + i as u8) as char)
                    .to_string()
                    .repeat(cnt[i] as usize),
            );
        }
        res
    }

    // 获取最大字典序字符串（降序排列）
    fn get_max_string(cnt: &[i32]) -> String {
        let total_len: usize = cnt.iter().map(|&c| c as usize).sum();
        let mut res = String::with_capacity(total_len);

        for i in (0..26).rev() {
            res.push_str(
                &((b'a' + i as u8) as char)
                    .to_string()
                    .repeat(cnt[i] as usize),
            );
        }
        res
    }
}

fn main() {
    let tests = vec![
        ("abc", "bba", "bca"),
        ("leet", "code", "eelt"),
        ("baba", "bbaa", ""),
    ];

    for (s, target, expected) in tests {
        assert_eq!(
            Solution::lex_greater_permutation(s.to_string(), target.to_string()),
            expected
        );
    }
}
