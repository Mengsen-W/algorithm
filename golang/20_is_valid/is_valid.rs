struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            // s 长度必须是偶数
            return false;
        }
        let mut st = vec![];
        for c in s.bytes() {
            match c {
                b'(' => st.push(b')'), // 入栈对应的右括号
                b'[' => st.push(b']'),
                b'{' => st.push(b'}'),
                _ => {
                    // c 是右括号
                    if st.pop() != Some(c) {
                        return false; // 没有左括号，或者左括号类型不对
                    }
                }
            }
        }
        st.is_empty() // 所有左括号必须匹配完毕
    }
}

fn main() {
    let tests = vec![
        ("()", true),
        ("()[]{}", true),
        ("(]", false),
        ("([])", true),
        ("([)]", false),
    ];

    for (input, expected) in tests {
        assert_eq!(Solution::is_valid(input.to_string()), expected);
    }
}
