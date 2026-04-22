/*
 * @Date: 2024-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-30
 * @FilePath: /algorithm/rust/2798_number_of_employees_who_met_target/number_of_employees_who_met_target.rs
 */

struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;

        for i in 0..hours.len() {
            if hours[i] >= target {
                ans += 1
            }
        }
        return ans;
    }
}

fn main() {
    let tests = vec![(vec![0, 1, 2, 3, 4], 2, 3), (vec![5, 1, 4, 2, 2], 6, 0)];

    for (hours, target, ans) in tests {
        assert_eq!(
            Solution::number_of_employees_who_met_target(hours, target),
            ans
        );
    }
}
