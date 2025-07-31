struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().reduce(|xor, x| xor ^ x).unwrap_or(0) == 0
    }
}

fn main() {
    let tests = vec![
        (vec![1, 1, 0], true),
        (vec![1, 1], true),
        (vec![1, 0], false),
    ];

    for (derived, expected) in tests {
        assert_eq!(Solution::does_valid_array_exist(derived), expected);
    }
}
