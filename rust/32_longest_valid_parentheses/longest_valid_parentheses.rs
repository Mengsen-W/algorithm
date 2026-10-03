struct Solution;

impl Solution {
    fn solve(iter: impl Iterator<Item = u8>, left_ch: u8) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut right = 0;

        for ch in iter {
            if ch == left_ch {
                left += 1;
            } else {
                right += 1;
            }
            if left < right {
                // 右括号太多了，重置计数器
                left = 0;
                right = 0;
            } else if left == right {
                ans = ans.max(right * 2);
            }
        }

        ans
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let ans1 = Self::solve(s.bytes(), b'(');
        let ans2 = Self::solve(s.bytes().rev(), b')');
        ans1.max(ans2)
    }
}

fn main() {
    let tests = vec![("(()", 2), (")()())", 4), ("", 0)];

    for (s, expected) in tests {
        assert_eq!(Solution::longest_valid_parentheses(s.to_string()), expected);
    }
}
