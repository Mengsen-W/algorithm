struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        let n = words.len();
        for i in 0..n {
            if i == 0 || groups[i] != groups[i - 1] {
                ans.push(words[i].clone());
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["e", "a", "b"], vec![0, 0, 1], vec!["e", "b"]),
        (
            vec!["a", "b", "c", "d"],
            vec![1, 0, 1, 1],
            vec!["a", "b", "c"],
        ),
    ];

    for (words, groups, expected) in tests {
        assert_eq!(
            Solution::get_longest_subsequence(
                words.iter().map(|s| s.to_string()).collect(),
                groups
            ),
            expected
        );
    }
}
