struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        (coordinate1.chars().nth(0).unwrap() as i32 - coordinate2.chars().nth(0).unwrap() as i32
            + coordinate1.chars().nth(1).unwrap() as i32
            - coordinate2.chars().nth(1).unwrap() as i32)
            % 2
            == 0
    }
}

fn main() {
    let tests = vec![("a1", "c3", true), ("a1", "h3", false)];

    for (coordinate1, coordinate2, ans) in tests {
        assert_eq!(
            Solution::check_two_chessboards(coordinate1.to_string(), coordinate2.to_string()),
            ans
        );
    }
}
