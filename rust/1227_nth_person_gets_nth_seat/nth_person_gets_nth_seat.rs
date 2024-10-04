struct Solution;

impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}

fn main() {
    let tests = vec![(1, 1.0), (2, 0.5)];

    for (n, ans) in tests {
        assert_eq!(Solution::nth_person_gets_nth_seat(n), ans);
    }
}
