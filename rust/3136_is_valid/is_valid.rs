struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut has_vowel = false;
        let mut has_consonant = false;

        for c in word.chars() {
            if c.is_ascii_alphabetic() {
                let lc = c.to_ascii_lowercase();
                if "aeiou".contains(lc) {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if !c.is_ascii_digit() {
                return false;
            }
        }

        has_vowel && has_consonant
    }
}

fn main() {
    let tests = vec![("234Adas", true), ("b3", false), ("a3$e", false)];

    for (test, expected) in tests {
        assert_eq!(Solution::is_valid(test.to_string()), expected);
    }
}
