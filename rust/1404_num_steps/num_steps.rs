struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let n = s.len();
        let mut ans = 0;
        // meet1 记录我们有没有遇见过字符 1
        let mut meet1 = false;
        // 从后向前遍历字符
        for (i, ch) in s.chars().rev().enumerate() {
            let pos = n - 1 - i; // 计算原始位置
            if ch == '0' {
                ans += if meet1 { 2 } else { 1 };
            } else {
                if !meet1 {
                    if pos != 0 {
                        ans += 2;
                    }
                    meet1 = true;
                } else {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("1101", 6), ("10", 1), ("1", 0)];

    for (s, expected) in tests {
        assert_eq!(Solution::num_steps(s.to_string()), expected);
    }
}
