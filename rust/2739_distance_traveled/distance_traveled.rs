/*
 * @Date: 2024-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-25
 * @FilePath: /algorithm/rust/2739_distance_traveled/distance_traveled.rs
 */

struct Solution;

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;
        let mut ans = 0;
        while main_tank >= 5 {
            main_tank -= 5;
            ans += 50;
            if additional_tank > 0 {
                additional_tank -= 1;
                main_tank += 1;
            }
        }
        ans + main_tank * 10
    }
}

fn main() {
    let tests = vec![(5, 10, 60), (1, 2, 10)];

    for (main_tank, additional_tank, ans) in tests {
        assert_eq!(Solution::distance_traveled(main_tank, additional_tank), ans);
    }
}
