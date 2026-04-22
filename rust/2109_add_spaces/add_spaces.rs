struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut count = 0;
        let mut iter = spaces.iter();
        let mut ans = String::new();
        let mut entry = iter.next();
        for c in s.chars() {
            if entry != None && *(entry.unwrap()) == count {
                ans.push(' ');
                entry = iter.next();
            }
            ans.push(c);
            count += 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            "LeetcodeHelpsMeLearn",
            vec![8, 13, 15],
            "Leetcode Helps Me Learn",
        ),
        ("icodeinpython", vec![1, 5, 7, 9], "i code in py thon"),
        ("spacing", vec![0, 1, 2, 3, 4, 5, 6], " s p a c i n g"),
    ];

    for (s, spaces, ans) in tests {
        assert_eq!(Solution::add_spaces(s.to_string(), spaces), ans.to_string());
    }
}
