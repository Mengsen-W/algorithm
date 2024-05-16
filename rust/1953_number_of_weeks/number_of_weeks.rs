/*
 * @Date: 2024-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-16
 * @FilePath: /algorithm/rust/1953_number_of_weeks/number_of_weeks.rs
 */

struct Solution;

impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        // 耗时最长工作所需周数
        let mut longest = 0;
        let mut rest: i64 = 0;
        for &count in milestones.iter() {
            longest = std::cmp::max(longest, count);
            rest += count as i64;
        }
        // 其余工作共计所需周数
        rest -= longest as i64;
        if longest as i64 > rest + 1 {
            // 此时无法完成所耗时最长的工作
            rest * 2 + 1
        } else {
            // 此时可以完成所有工作
            longest as i64 + rest
        }
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 6), (vec![5, 2, 1], 7)];

    for (milestones, ans) in tests {
        assert_eq!(Solution::number_of_weeks(milestones), ans);
    }
}
