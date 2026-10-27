struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let n = n as usize;
        let m = m as usize;
        let mut ans = vec![];
        ans.push(".".repeat(n));
        for _ in 1..m {
            ans.push("#".repeat(n - 1) + ".");
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (2, 3, vec!["..#", "#.."]),
        (3, 3, vec!["..#", "#..", "##."]),
        (1, 4, vec!["...."]),
    ];

    for (m, n, expected) in tests {
        assert_eq!(Solution::create_grid(m, n), expected);
    }
}
