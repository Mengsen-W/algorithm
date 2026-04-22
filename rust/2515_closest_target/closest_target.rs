struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut ans = n;

        for (i, word) in words.iter().enumerate() {
            if word == &target {
                let dist = (i as i32 - start_index).abs();
                let distance = dist.min(n - dist);
                ans = ans.min(distance)
            }
        }

        if ans < n {
            ans
        } else {
            -1
        }
    }
}

fn main() {
    let tests = vec![
        ( vec![ "hello", "i", "am", "leetcode", "hello" ], "hello", 1, 1 ),
        ( vec![ "a", "b", "leetcode" ], "leetcode", 0, 1 ),
        ( vec![ "i", "eat", "leetcode" ], "ate", 0, -1 ),
    ];

    for (words, target, start_index, ans) in tests {
        assert_eq!(Solution::closest_target(words.iter().map(|s| s.to_string()).collect(), target.to_string(), start_index), ans);
    }
}
