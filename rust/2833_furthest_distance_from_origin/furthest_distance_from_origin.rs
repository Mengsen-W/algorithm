struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut l, mut r, mut b) = (0, 0, 0);
        for c in moves.chars() {
            match c {
                'L' => l += 1,
                'R' => r += 1,
                '_' => b += 1,
                _ => (),
            }
        }
        (l as i32 - r as i32).abs() + b as i32
    }
}

fn main() {
    let tests = vec![("L_RL__R", 3), ("_R__LL_", 5), ("_______", 7)];

    for (moves, ans) in tests {
        assert_eq!(
            Solution::furthest_distance_from_origin(moves.to_string()),
            ans
        );
    }
}
