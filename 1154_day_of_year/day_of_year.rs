/*
 * @Date: 2021-12-21 01:20:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-21 01:50:24
 */

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

fn main() {
    assert_eq!(day_of_year("2019-01-09".to_string()), 9);
    assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    assert_eq!(day_of_year("2003-03-01".to_string()), 60);
    assert_eq!(day_of_year("2004-03-01".to_string()), 61);
}
