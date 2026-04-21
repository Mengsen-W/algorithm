struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k == k {
            (n ^ k).count_ones() as i32
        } else {
            -1
        }
    }
}

fn main() {
    let tests = vec![(13, 4, 2), (21, 21, 0), (14, 13, -1)];

    for (n, k, ans) in tests {
        assert_eq!(Solution::min_changes(n, k), ans);
    }
}
