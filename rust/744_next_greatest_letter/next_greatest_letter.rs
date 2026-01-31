struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        *letters.iter().find(|&c| c > &target).unwrap_or(&letters[0])
    }
}

fn main() {
    let tests = vec![
        (vec!['c', 'f', 'j'], 'a', 'c'),
        (vec!['c', 'f', 'j'], 'c', 'f'),
        (vec!['x', 'x', 'y', 'y'], 'z', 'x'),
    ];

    for (letters, target, expected) in tests {
        assert_eq!(Solution::next_greatest_letter(letters, target), expected);
    }
}
