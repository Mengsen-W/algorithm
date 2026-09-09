struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut p: i64 = 1000;
        let mut res: i64 = 0;
        while p <= n {
            res += n - p + 1;
            p *= 1000;
        }
        res
    }
}

fn main() {
    let tests = vec![(1002, 3), (998, 0)];

    for (n, expected) in tests {
        assert_eq!(Solution::count_commas(n), expected);
    }
}
