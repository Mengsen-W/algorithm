struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut k: i64 = 1;
        loop {
            let x: i64 = num1 as i64 - num2 as i64 * k;
            if x < k {
                return -1;
            }
            if k >= x.count_ones() as i64 {
                return k as i32;
            }
            k += 1;
        }
    }
}

fn main() {
    let tests = vec![(3, -2, 3), (5, 7, -1)];

    for (num1, num2, expected) in tests {
        assert_eq!(Solution::make_the_integer_zero(num1, num2), expected);
    }
}
