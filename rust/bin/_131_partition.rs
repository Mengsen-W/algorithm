/*
 * @Date: 2021-03-07 09:30:25
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-07 09:49:33
 */

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &[u8], lo: usize, hi: usize) -> bool {
            let mut i = lo;
            let mut j = hi;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn dfs(arr: &mut Vec<String>, s: &str, i: usize, answer: &mut Vec<Vec<String>>) {
            if i == s.len() {
                answer.push(arr.clone());
                return;
            }
            for j in i..s.len() {
                if is_palindrome(s.as_bytes(), i, j) {
                    arr.push(s[i..=j].to_string());
                    dfs(arr, s, j + 1, answer);
                    arr.pop();
                }
            }
        }

        let mut answer = Vec::new();
        dfs(&mut Vec::new(), &s, 0, &mut answer);
        answer
    }
}

fn main() {
    let tests = vec![
        (
            "aab".to_string(),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
        ),
        ("a".to_string(), vec![vec!["a".to_string()]]),
    ];

    for (s, ans) in tests {
        assert_eq!(Solution::partition(s), ans);
    }
}
