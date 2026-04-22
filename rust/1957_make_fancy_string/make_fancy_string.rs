struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = Vec::new(); // 删除后的字符串

        // 遍历 s 模拟删除过程
        for ch in s.chars() {
            let n = res.len();
            if n >= 2 && res[n - 1] == ch && res[n - 2] == ch {
                // 如果 res 最后两个字符与当前字符均相等，则不添加
                continue;
            }
            // 反之则添加
            res.push(ch);
        }
        res.into_iter().collect()
    }
}

fn main() {
    let tests = vec![
        ("leeetcode", "leetcode"),
        ("aaabaaaa", "aabaa"),
        ("aab", "aab"),
    ];

    for (s, expected) in tests {
        assert_eq!(Solution::make_fancy_string(s.to_string()), expected);
    }
}
