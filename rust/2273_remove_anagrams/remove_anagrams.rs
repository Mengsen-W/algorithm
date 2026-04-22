struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![words[0].clone()]; // 结果数组
        let n = words.len();

        // 判断两个单词是否为字母异位词
        fn compare(word1: &String, word2: &String) -> bool {
            let mut freq = [0; 26];
            for ch in word1.chars() {
                freq[(ch as u8 - b'a') as usize] += 1;
            }
            for ch in word2.chars() {
                freq[(ch as u8 - b'a') as usize] -= 1;
            }
            freq.iter().all(|&x| x == 0)
        }

        for i in 1..n {
            if !compare(&words[i], &words[i - 1]) {
                res.push(words[i].clone());
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["abba", "baba", "bbaa", "cd", "cd"], vec!["abba", "cd"]),
        (vec!["a", "b", "c", "d", "e"], vec!["a", "b", "c", "d", "e"]),
    ];

    for (words, expected) in tests {
        assert_eq!(
            Solution::remove_anagrams(words.iter().map(|s| s.to_string()).collect()),
            expected.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
    }
}
