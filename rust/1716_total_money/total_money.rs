/*
 * @Date: 2022-01-15 01:43:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-15 02:12:18
 */

pub fn total_money(n: i32) -> i32 {
    let week_number = n / 7;
    let first_week_money = (1 + 7) * 7 / 2;
    let last_week_money = first_week_money + 7 * (week_number - 1);
    let week_money = (first_week_money + last_week_money) * week_number / 2;
    let day_number = n % 7;
    let first_day_money = 1 + week_number;
    let last_day_money = first_day_money + day_number - 1;
    let day_money = (first_day_money + last_day_money) * day_number / 2;
    week_money + day_money
}

fn main() {
    assert_eq!(total_money(4), 10);
    assert_eq!(total_money(10), 37);
    assert_eq!(total_money(20), 96);
}
