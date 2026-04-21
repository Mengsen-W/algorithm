struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut ans = 0;
        for c in commands {
            match c.chars().next().unwrap() {
                'U' => ans -= n,
                'D' => ans += n,
                'L' => ans -= 1,
                _ => ans += 1,
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (2, vec!["RIGHT", "DOWN"], 3),
        (3, vec!["DOWN", "RIGHT", "UP"], 1),
    ];

    for (n, commands, ans) in tests {
        assert_eq!(
            Solution::final_position_of_snake(n, commands.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}
