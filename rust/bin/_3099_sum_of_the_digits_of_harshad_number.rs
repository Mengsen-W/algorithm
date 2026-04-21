struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut s = 0;
        let mut y = x;
        while y != 0 {
            s += y % 10;
            y /= 10;
        }
        if x % s != 0 {
            -1
        } else {
            s
        }
    }
}

fn main() {
    let tests = vec![(18, 9), (23, -1)];

    for (x, ans) in tests {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(x), ans);
    }
}
