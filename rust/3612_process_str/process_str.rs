struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result: Vec<char> = Vec::new();
        for ch in s.chars() {
            match ch {
                '*' => {
                    result.pop();
                }
                '#' => {
                    let mut copy = result.clone();
                    result.append(&mut copy);
                }
                '%' => {
                    result.reverse();
                }
                c => result.push(c),
            }
        }
        result.into_iter().collect()
    }
}

fn main() {
    let tests = vec![("a#b%*", "ba"), ("z*#", "")];

    for (s, expected) in tests {
        assert_eq!(Solution::process_str(s.to_string()), expected);
    }
}
