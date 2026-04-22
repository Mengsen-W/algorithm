struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right).fold(0, |ret, i| ret + (665772 >> i.count_ones() & 1))
    }
}

fn main() {
    let tests = vec![(6, 10, 4), (10, 15, 5)];

    for (left, right, ans) in tests {
        assert_eq!(Solution::count_prime_set_bits(left, right), ans);
    }
}
