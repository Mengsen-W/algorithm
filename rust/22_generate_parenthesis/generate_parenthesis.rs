struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // 目前填了 i 个括号
        // 这 i 个括号中的左括号个数 - 右括号个数 = balance
        fn dfs(i: usize, balance: usize, n: usize, path: &mut Vec<usize>, ans: &mut Vec<String>) {
            if path.len() == n {
                let mut s = vec![b')'; n * 2];
                for &mut j in path {
                    s[j] = b'(';
                }
                ans.push(unsafe { String::from_utf8_unchecked(s) });
                return;
            }
            // 枚举填 right=0,1,2,...,balance 个右括号
            for right in 0..=balance {
                // 先填 right 个右括号，然后填 1 个左括号，记录左括号的下标 i+right
                path.push(i + right);
                dfs(i + right + 1, balance - right + 1, n, path, ans);
                path.pop(); // 恢复现场
            }
        }

        let mut ans = vec![];
        let mut path = vec![];
        dfs(0, 0, n as usize, &mut path, &mut ans);
        ans
    }
}

fn main() {
    let tests = vec![
        (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        (1, vec!["()"]),
    ];

    for (n, expected) in tests {
        assert_eq!(Solution::generate_parenthesis(n), expected);
    }
}
