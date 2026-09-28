struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(mx, cnt), c| match c {
                '(' => (mx.max(cnt + 1), cnt + 1),
                ')' => (mx, cnt - 1),
                _ => (mx, cnt),
            })
            .0
    }
}

fn main() {
    let tests = vec![
        ("(1+(2*3)+((8)/4))+1", 3),
        ("(1)+((2))+(((3)))", 3),
        ("1+(2*3)/(2-1)", 1),
        ("1", 0),
    ];

    for (s, ans) in tests {
        assert_eq!(Solution::max_depth(s.to_string()), ans);
    }
}
