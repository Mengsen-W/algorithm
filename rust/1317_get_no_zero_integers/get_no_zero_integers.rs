struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..n {
            let b = n - a;
            if !a.to_string().contains('0') && !b.to_string().contains('0') {
                return vec![a, b];
            }
        }
        vec![]
    }
}

fn main() {
    let tests = vec![
        (2, vec![1, 1]),
        (11, vec![2, 9]),
        (10000, vec![1, 9999]),
        (69, vec![1, 68]),
        (1010, vec![11, 999]),
    ];

    for (n, expected) in tests {
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }
}
