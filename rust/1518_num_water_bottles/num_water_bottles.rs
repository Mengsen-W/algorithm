struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let cnt = num_bottles / (num_exchange - 1);
        if num_bottles % (num_exchange - 1) == 0 {
            num_bottles + cnt - 1
        } else {
            num_bottles + cnt
        }
    }
}

fn main() {
    let tests = vec![(9, 3, 13), (15, 4, 19), (5, 5, 6), (2, 3, 2)];

    for (num_bottles, num_exchange, expected) in tests {
        assert_eq!(
            Solution::num_water_bottles(num_bottles, num_exchange),
            expected
        );
    }
}
