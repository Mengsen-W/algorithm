struct Solution;

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let start_ = (start - 1).to_string();
        let finish_ = finish.to_string();
        // let val1 = Self::calculate(&finish_, &s, limit);
        // let val2 = Self::calculate(&start_, &s, limit);
        Self::calculate(&finish_, &s, limit) - Self::calculate(&start_, &s, limit)
    }

    pub fn calculate(x: &str, s: &str, limit: i32) -> i64 {
        if x.len() < s.len() {
            return 0;
        }
        if x.len() == s.len() {
            return if x >= s { 1 } else { 0 };
        }

        let suffix = &x[x.len() - s.len()..];
        let mut count = 0i64;
        let pre_len = x.len() - s.len();

        for i in 0..pre_len {
            let digit = x.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            if limit < digit {
                count += ((limit + 1) as i64).pow((pre_len - i) as u32);
                return count;
            }
            count += (digit as i64) * ((limit + 1) as i64).pow((pre_len - 1 - i) as u32);
        }
        if suffix >= s {
            count += 1;
        }
        count
    }
}

fn main() {
    let tests = vec![
        (1, 6000, 4, "124", 5),
        (15, 215, 6, "10", 2),
        (1000, 2000, 4, "3000", 0),
    ];

    for (start, finish, limit, s, ans) in tests {
        assert_eq!(
            Solution::number_of_powerful_int(start, finish, limit, s.to_string()),
            ans
        );
    }
}
