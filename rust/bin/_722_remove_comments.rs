/*
 * @Date: 2023-08-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-03
 * @FilePath: /algorithm/rust/722_remove_comments/remove_comments.rs
 */

struct Solution;
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut t: Vec<String> = Vec::new();
        let mut block_comment = false;

        for s in &source {
            let m = s.len();
            let mut i = 0;
            while i < m {
                if block_comment {
                    if i + 1 < m && &s[i..i + 2] == "*/" {
                        block_comment = false;
                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    if i + 1 < m && &s[i..i + 2] == "/*" {
                        block_comment = true;
                        i += 2;
                    } else if i + 1 < m && &s[i..i + 2] == "//" {
                        break;
                    } else {
                        t.push(s.chars().nth(i).unwrap().to_string());
                        i += 1;
                    }
                }
            }
            if !block_comment && !t.is_empty() {
                ans.push(t.join(""));
                t.clear();
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                "/*Test program */",
                "int main()",
                "{ ",
                "  // variable declaration ",
                "int a, b, c;",
                "/* This is a test",
                "   multiline  ",
                "   comment for ",
                "   testing */",
                "a = b + c;",
                "}",
            ],
            vec!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"],
        ),
        (vec!["a/*comment", "line", "more_comment*/b"], vec!["ab"]),
    ];

    for (source, ans) in tests {
        assert_eq!(
            Solution::remove_comments(source.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}
