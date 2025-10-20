struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(0, |mut x: i32, op: &String| {
            match op.chars().nth(1).unwrap() {
                '-' => x -= 1,
                _ => x += 1,
            }
            x
        })
    }
}

fn main() {
    let tests = vec![
        (vec!["--X", "X++", "X++"], 1),
        (vec!["++X", "++X", "X++"], 3),
        (vec!["X++", "++X", "--X", "X--"], 0),
    ];

    for (operations, ans) in tests {
        assert_eq!(
            Solution::final_value_after_operations(
                operations.iter().map(|s| s.to_string()).collect()
            ),
            ans
        );
    }
}
