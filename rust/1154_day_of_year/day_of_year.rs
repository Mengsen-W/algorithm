/*
 * @Date: 2021-12-21 01:20:49
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-01
 */

struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        const DAY_OF_TOTAL: &[i32] = &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
        let date_arr: Vec<&str> = date.as_str().split("-").collect();
        let year: i32 = date_arr[0].parse().unwrap();
        let month: i32 = date_arr[1].parse().unwrap();
        let day: i32 = date_arr[2].parse().unwrap();

        let mut ret = DAY_OF_TOTAL[(month - 1) as usize] + day;

        if month > 2 && (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) {
            ret += 1;
        }
        ret
    }
}

fn main() {
    let tests = vec![
        ("2019-01-09", 9),
        ("2019-02-10", 41),
        ("2003-03-01", 60),
        ("2004-03-01", 61),
    ];

    for (date, ans) in tests {
        assert_eq!(Solution::day_of_year(date.to_string()), ans);
    }
}
