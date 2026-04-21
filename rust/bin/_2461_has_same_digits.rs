struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let n = s.len();
        let mut arr: Vec<u8> = s.bytes().collect();

        for i in 1..=n - 2 {
            for j in 0..=n - 1 - i {
                let digit1 = (arr[j] - b'0') as u8;
                let digit2 = (arr[j + 1] - b'0') as u8;
                arr[j] = ((digit1 + digit2) % 10) + b'0';
            }
        }
        arr[0] == arr[1]
    }
}

fn main() {
    let tests = vec![("3902", true), ("34789", false)];

    for (s, expected) in tests {
        assert_eq!(Solution::has_same_digits(s.to_string()), expected);
    }
}
