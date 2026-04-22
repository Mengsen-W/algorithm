struct Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        if x.min(y / 4) % 2 != 0 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}

fn main() {
    let tests = vec![(2, 7, "Alice"), (4, 11, "Bob")];

    for (x, y, ans) in tests {
        assert_eq!(Solution::losing_player(x, y), ans.to_string());
    }
}
