/*
 * @Date: 2023-10-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-23
 * @FilePath: /algorithm/rust/2678_count_seniors/count_seniors.rs
 */

struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter_map(|s| s[11..13].parse::<i32>().ok())
            .filter(|&age| age > 60)
            .count() as i32
    }
}

fn main() {
    let tests = vec![
        (
            vec!["7868190130M7522", "5303914400F9211", "9273338290F4010"],
            2,
        ),
        (vec!["1313579440F2036", "2921522980M5644"], 0),
    ];

    for (details, ans) in tests {
        assert_eq!(
            Solution::count_seniors(details.into_iter().map(String::from).collect()),
            ans
        );
    }
}
