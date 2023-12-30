/*
 * @Date: 2022-01-03 01:24:17
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-30
 */

struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        const WEEK: [&str; 7] = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        const MONTH_DAY: [i32; 11] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];

        let mut days = 365 * (year - 1971) + (year - 1969) / 4;
        for d in &MONTH_DAY[0..(month - 1) as usize] {
            days += d;
        }
        if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) && month >= 3 {
            days += 1;
        }
        days += day;
        WEEK[((days + 3) % 7) as usize].to_string()
    }
}

fn main() {
    let tests = vec![
        (31, 8, 2019, "Saturday"),
        (18, 7, 1999, "Sunday"),
        (15, 8, 1993, "Sunday"),
        (29, 2, 2016, "Monday"),
    ];

    for (day, month, year, ans) in tests {
        assert_eq!(Solution::day_of_the_week(day, month, year), ans);
    }
}
