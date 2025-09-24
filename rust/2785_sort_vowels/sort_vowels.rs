struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let vowel_set: std::collections::HashSet<_> = vowels.iter().cloned().collect();

        let mut tmp: Vec<char> = s.chars().filter(|c| vowel_set.contains(c)).collect();

        tmp.sort();

        let mut result = String::with_capacity(s.len());
        let mut idx = 0;

        for c in s.chars() {
            if vowel_set.contains(&c) {
                result.push(tmp[idx]);
                idx += 1;
            } else {
                result.push(c);
            }
        }

        result
    }
}

fn main() {
    let tests = vec![("lEetcOde", "lEOtcede"), ("lYmpH", "lYmpH")];

    for (input, expected) in tests {
        assert_eq!(Solution::sort_vowels(input.to_string()), expected);
    }
}
