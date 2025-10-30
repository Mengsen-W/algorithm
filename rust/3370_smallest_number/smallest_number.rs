struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut x = 1;
        while x < n {
            x = x * 2 + 1;
        }
        x
    }
}

fn main() {
    let tests = vec![(5, 7), (10, 15), (3, 3)];

    for (n, expected) in tests {
        assert_eq!(Solution::smallest_number(n), expected, "n: {}", n);
    }
}
