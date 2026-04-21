struct Solution;

impl Solution {
    fn binary(x: i32) -> String {
        let mut s = String::new();
        let mut x = x;
        while x != 0 {
            s.push((x & 1).to_string().chars().next().unwrap());
            x >>= 1;
        }
        s.chars().rev().collect()
    }

    pub fn convert_date_to_binary(date: String) -> String {
        let year: i32 = date[0..4].parse().unwrap();
        let month: i32 = date[5..7].parse().unwrap();
        let day: i32 = date[8..10].parse().unwrap();
        format!(
            "{}-{}-{}",
            Self::binary(year),
            Self::binary(month),
            Self::binary(day)
        )
    }
}

fn main() {
    let tests = vec![
        ("2080-02-29", "100000100000-10-11101"),
        ("1900-01-01", "11101101100-1-1"),
    ];

    for (date, ans) in tests {
        assert_eq!(
            Solution::convert_date_to_binary(date.to_string()),
            ans.to_string()
        );
    }
}
