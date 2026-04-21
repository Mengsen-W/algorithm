struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let (mut north, mut south, mut east, mut west) = (0, 0, 0, 0);
        for c in s.chars() {
            match c {
                'N' => north += 1,
                'S' => south += 1,
                'E' => east += 1,
                'W' => west += 1,
                _ => (),
            }
            let times1 = min(min(north, south), k); // modification times for N and S
            let times2 = min(min(east, west), k - times1); // modification times for E and W
            let current = Self::count(north, south, times1) + Self::count(east, west, times2);
            ans = max(ans, current);
        }
        ans
    }

    fn count(drt1: i32, drt2: i32, times: i32) -> i32 {
        (drt1 - drt2).abs() + times * 2
    } // Calculate modified Manhattan distance
}

fn main() {
    let tests = vec![("NWSE", 1, 3), ("NSWWEW", 3, 6)];

    for (s, k, ans) in tests {
        assert_eq!(Solution::max_distance(s.to_string(), k), ans);
    }
}
