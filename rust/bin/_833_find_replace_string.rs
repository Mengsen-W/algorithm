/*
 * @Date: 2023-08-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-15
 * @FilePath: /algorithm/rust/833_find_replace_string/find_replace_string.rs
 */

struct Solution;

impl Solution {
    pub fn find_replace_string(
        mut s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let n = indices.len();
        let mut arr = vec![];

        for i in 0..n {
            arr.push((indices[i] as usize, i));
        }

        arr.sort_unstable();

        for (i, j) in arr.into_iter().rev() {
            let source = &sources[j];
            let m = source.len();
            let target = &targets[j];

            if i + m <= s.len() && &s[i..i + m] == source {
                s.replace_range(i..i + m, target);
            }
        }

        s
    }
}

fn main() {
    let tests = vec![
        (
            "abcd".to_string(),
            vec![0, 2],
            vec!["a", "cd"].iter().map(|s| s.to_string()).collect(),
            vec!["eee", "ffff"].iter().map(|s| s.to_string()).collect(),
            "eeebffff".to_string(),
        ),
        (
            "abcd".to_string(),
            vec![0, 2],
            vec!["ab", "ec"].iter().map(|s| s.to_string()).collect(),
            vec!["eee", "ffff"].iter().map(|s| s.to_string()).collect(),
            "eeecd".to_string(),
        ),
    ];

    for (s, indices, sources, targets, ans) in tests {
        assert_eq!(
            Solution::find_replace_string(s, indices, sources, targets),
            ans
        );
    }
}
