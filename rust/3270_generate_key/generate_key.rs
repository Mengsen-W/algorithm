struct Solution;

impl Solution {
    pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
        use std::cmp::min;
        let mut key = 0;
        let mut p = 1;
        while num1 > 0 && num2 > 0 && num3 > 0 {
            key += min(min(num1 % 10, num2 % 10), num3 % 10) * p;
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
            p *= 10;
        }
        key
    }
}

fn main() {
    let tests = vec![(1, 10, 1000, 0), (987, 879, 798, 777), (1, 2, 3, 1)];

    for (num1, num2, num3, ans) in tests {
        assert_eq!(Solution::generate_key(num1, num2, num3), ans);
    }
}
