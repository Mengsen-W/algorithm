/*
 * @Date: 2023-09-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-08
 * @FilePath: /algorithm/rust/2651_find_delayed_arrival_time/find_delayed_arrival_time.rs
 */

struct Solution;
impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

fn main() {
    let tests = vec![(15, 5, 20), (13, 11, 0)];

    for (arrival_time, delayed_time, expect) in tests {
        assert_eq!(
            Solution::find_delayed_arrival_time(arrival_time, delayed_time),
            expect
        );
    }
}
