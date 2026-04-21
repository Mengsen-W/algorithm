struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res = 0;
        for c in n.chars() {
            res = res.max(c as i32 - '0' as i32);
        }
        res
    }
}

fn main() {
    let tests = vec![("32", 3), ("82734", 8), ("27346209830709182346", 9)];

    for (n, expected) in tests {
        assert_eq!(Solution::min_partitions(n.to_string()), expected);
    }
}
