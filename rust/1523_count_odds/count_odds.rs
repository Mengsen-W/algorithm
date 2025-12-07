struct Solution;

impl Solution {
    fn pre(x: i32) -> i32 {
        (x + 1) >> 1
    }
    
    pub fn count_odds(low: i32, high: i32) -> i32 {
        Self::pre(high) - Self::pre(low - 1)
    }
}

fn main() {
    let tests = vec![
        (3, 7, 3),
        (8, 10, 1),
    ];

    for (low, high, ans) in tests {
        assert_eq!(Solution::count_odds(low, high), ans);
    }
}
