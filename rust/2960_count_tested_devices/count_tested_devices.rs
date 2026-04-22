/*
 * @Date: 2024-05-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-10
 * @FilePath: /algorithm/rust/2960_count_tested_devices/count_tested_devices.rs
 */

struct Solution;

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut need = 0;
        for &battery in battery_percentages.iter() {
            if battery > need {
                need += 1;
            }
        }
        need
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 2, 1, 3], 3), (vec![0，1，2]，2)];

    for (battery_percentages, ans) in tests {
      assert_eq!(Solution::count_tested_devices(battery_percentages), ans);
    }
}
