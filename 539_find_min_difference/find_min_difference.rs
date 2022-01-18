/*
 * @Date: 2022-01-18 02:13:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-18 03:09:08
 */

struct Solution;

impl Solution {
    pub fn find_min_difference(mut time_points: Vec<String>) -> i32 {
        let n = time_points.len();
        if n > 1440 {
            return 0;
        }
        time_points.sort();
        let mut ans = i32::MAX;
        let t0_minutes = Self::get_minutes(&time_points[0]);
        let mut pre_minutes = t0_minutes;
        for i in 1..n {
            let minutes = Self::get_minutes(&time_points[i]);
            ans = ans.min(minutes - pre_minutes);
            pre_minutes = minutes;
        }
        ans = ans.min(t0_minutes + 1440 - pre_minutes);
        ans
    }

    fn get_minutes(t: &String) -> i32 {
        let t = t
            .chars()
            .map(|c| (c as u8 - b'0') as i32)
            .collect::<Vec<i32>>();
        (t[0] * 10 + t[1]) * 60 + t[3] * 10 + t[4]
    }
}

fn main() {
    assert_eq!(
        Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
        1,
    );
    assert_eq!(
        Solution::find_min_difference(vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string()
        ]),
        0,
    );
}
