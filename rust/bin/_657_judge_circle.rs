struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;

        for ch in moves.chars() {
            match ch {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => continue,
            }
        }

        x == 0 && y == 0
    }
}

fn main() {
    let tests = vec![("UD", true), ("LL", false)];

    for (moves, expected) in tests {
        assert_eq!(Solution::judge_circle(moves.to_string()), expected);
    }
}
