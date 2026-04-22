/*
 * @Date: 2022-11-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-01
 * @FilePath: /algorithm/1662_array_strings_are_equal/array_strings_are_equal.rs
 */

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let (word1, word2): (Vec<Vec<char>>, Vec<Vec<char>>) = (
        word1.iter().map(|s| s.chars().collect()).collect(),
        word2.iter().map(|s| s.chars().collect()).collect(),
    );
    let (mut p1, mut p2, mut i, mut j) = (0, 0, 0, 0);
    while p1 < word1.len() && p2 < word2.len() {
        if word1[p1][i] != word2[p2][j] {
            return false;
        }
        i += 1;
        if i == word1[p1].len() {
            p1 += 1;
            i = 0;
        }
        j += 1;
        if j == word2[p2].len() {
            p2 += 1;
            j = 0;
        }
    }
    return p1 == word1.len() && p2 == word2.len();
}

fn main() {
    {
        let word1 = vec!["ab", "c"].iter().map(|s| s.to_string()).collect();
        let word2 = vec!["a", "bc"].iter().map(|s| s.to_string()).collect();
        assert!(array_strings_are_equal(word1, word2));
    }

    {
        let word1 = vec!["a", "cb"].iter().map(|s| s.to_string()).collect();
        let word2 = vec!["ab", "c"].iter().map(|s| s.to_string()).collect();
        assert!(!array_strings_are_equal(word1, word2));
    }

    {
        let word1 = vec!["abc", "d", "defg"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let word2 = vec!["abcddefg"].iter().map(|s| s.to_string()).collect();
        assert!(array_strings_are_equal(word1, word2));
    }
}
