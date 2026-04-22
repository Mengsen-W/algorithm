struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        fn cal(x: i32) -> i64 {
            if x < 0 {
                0
            } else {
                x as i64 * (x - 1) as i64 / 2
            }
        }
        cal(n + 2) - 3 * cal(n - limit + 1) + 3 * cal(n - (limit + 1) * 2 + 2)
            - cal(n - 3 * (limit + 1) + 2)
    }
}

fn main() {
    let tests = vec![(5, 2, 3), (3, 3, 10)];

    for (n, limit, ans) in tests {
        assert_eq!(Solution::distribute_candies(n, limit), ans);
    }
}
