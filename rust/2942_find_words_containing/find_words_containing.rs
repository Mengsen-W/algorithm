struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut res = Vec::new();
        let n = words.len();
        for i in 0..n {
            if words[i].contains(x) {
                res.push(i as i32);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["leet", "code"], 'e', vec![0, 1]),
        (vec!["abc", "bcd", "aaaa", "cbc"], 'a', vec![0, 2]),
        (vec!["abc", "bcd", "aaaa", "cbc"], 'z', vec![]),
    ];

    for (words, x, ans) in tests {
        assert_eq!(
            Solution::find_words_containing(words.iter().map(|s| s.to_string()).collect(), x),
            ans
        );
    }
}
