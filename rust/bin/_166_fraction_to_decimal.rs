struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        use std::collections::HashMap;
        if numerator == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            s.push('-');
        }
        let (numerator, denominator) = ((numerator as i64).abs(), (denominator as i64).abs());
        let (integer, mut remainder) = (numerator / denominator, numerator % denominator);
        s.push_str(&integer.to_string());
        if remainder != 0 {
            s.push('.');
        }
        let mut map = HashMap::new();
        while remainder != 0 && !map.contains_key(&remainder) {
            map.insert(remainder, s.len());
            remainder *= 10;
            s.push_str(&(remainder / denominator).to_string());
            remainder %= denominator;
        }
        if let Some(pos) = map.get(&remainder) {
            s.insert(*pos, '(');
            s.push(')');
        }
        s
    }
}

fn main() {
    let tests = vec![
        (1, 2, "0.5"),
        (2, 1, "2"),
        (4, 333, "0.(012)"),
        (1, 5, "0.2"),
    ];

    for (numerator, denominator, expected) in tests {
        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            expected
        );
    }
}
