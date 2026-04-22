struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut ans = 0;
        let mut n = n;
        while n != 0 {
            ans ^= n;
            n >>= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(3, 2), (6, 4)];

    for (n, ans) in tests {
        assert_eq!(Solution::minimum_one_bit_operations(n), ans);
    }
}
