struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let n = target.len();
        let mut ans = target[0];
        for i in 1..n {
            ans += std::cmp::max(target[i] - target[i - 1], 0);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 2, 1], 3),
        (vec![3, 1, 1, 2], 4),
        (vec![3, 1, 5, 4, 2], 7),
        (vec![1, 1, 1, 1], 1),
    ];

    for (target, expected) in tests {
        assert_eq!(Solution::min_number_operations(target), expected);
    }
}
