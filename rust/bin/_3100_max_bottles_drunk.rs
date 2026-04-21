struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut empty = num_bottles;
        while empty >= num_exchange {
            ans += 1;
            empty -= num_exchange - 1;
            num_exchange += 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(13, 6, 15), (10, 3, 13)];

    for (num_bottles, num_exchange, ans) in tests {
        assert_eq!(Solution::max_bottles_drunk(num_bottles, num_exchange), ans);
    }
}
