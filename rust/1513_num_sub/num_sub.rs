struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MODULO: i64 = 1000000007;
        let mut total: i64 = 0;
        let mut consecutive: i64 = 0;
        for c in s.chars() {
            if c == '0' {
                total += consecutive * (consecutive + 1) / 2;
                total %= MODULO;
                consecutive = 0;
            } else {
                consecutive += 1;
            }
        }
        total += consecutive * (consecutive + 1) / 2;
        total %= MODULO;
        total as i32
    }
}

fn main() {
    let tests = vec![("0110111", 9), ("101", 2), ("111111", 21), ("000", 0)];

    for (s, ans) in tests {
        assert_eq!(Solution::num_sub(s.to_string()), ans);
    }
}
