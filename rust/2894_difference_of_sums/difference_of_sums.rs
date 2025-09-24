struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let k = n / m;
        n * (n + 1) / 2 - k * (k + 1) * m
    }
}

fn main() {
    let tests = vec![(10, 3, 19), (5, 6, 15), (5, 1, -15)];

    for (n, m, ans) in tests {
        assert_eq!(Solution::difference_of_sums(n, m), ans);
    }
}
