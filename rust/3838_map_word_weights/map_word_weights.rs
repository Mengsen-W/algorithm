struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut ans = String::with_capacity(words.len());

        for word in words {
            let mut s = 0;
            for c in word.bytes() {
                s += weights[(c - b'a') as usize];
            }
            ans.push((b'z' - (s % 26) as u8) as char);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec!["abcd", "def", "xyz"],
            vec![
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
            ],
            "rij",
        ),
        (
            vec!["a", "b", "c"],
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            "yyy",
        ),
    ];

    for (words, weights, expected) in tests {
        assert_eq!(
            Solution::map_word_weights(words.iter().map(|s| s.to_string()).collect(), weights),
            expected.to_string()
        );
    }
}
