struct Solution;

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let mut num = n;

        while num > 0 {
            let x = num % 10;
            if x > first {
                second = first;
                first = x;
            } else if x > second {
                second = x;
            }
            num /= 10;
        }

        first * second
    }
}

fn main() {
    let tests = vec![(31, 3), (22, 4), (124, 8)];

    for (n, expected) in tests {
        assert_eq!(Solution::max_product(n), expected);
    }
}
