struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut i = 0;
        let len = colors.len();
        let mut ret = 0;
        let colors = colors.chars().collect::<Vec<char>>();

        while i < len {
            let ch = colors[i];
            let mut max_value = 0;
            let mut sum = 0;

            while i < len && colors[i] == ch {
                max_value = max_value.max(needed_time[i]);
                sum += needed_time[i];
                i += 1;
            }
            ret += sum - max_value;
        }
        ret
    }
}

fn main() {
    let tests = vec![
        ("abaac", vec![1, 2, 3, 4, 5], 3),
        ("abc", vec![1, 1, 1], 0),
        ("aabaa", vec![1, 2, 3, 4, 1], 2),
    ];

    for (colors, needed_time, ans) in tests {
        assert_eq!(Solution::min_cost(colors.to_string(), needed_time), ans);
    }
}
