struct Solution;

impl Solution {
    fn reverse(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res = res * 10 + n % 10;
            n /= 10;
        }
        res
    }

    fn mirror_distance(n: i32) -> i32 {
        (n - Self::reverse(n)).abs()
    }
}

fn main() {
    let tests = vec![(25, 27), (10, 9), (7, 0)];

    for (n, expected) in tests {
        assert_eq!(Solution::mirror_distance(n), expected);
    }
}
