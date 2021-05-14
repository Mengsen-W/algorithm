/*
 * @Date: 2021-05-14 08:44:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-14 09:05:30
 */

const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
fn int_to_roman(num: i32) -> String {
    let num = num as usize;
    format!(
        "{}{}{}{}",
        THOUSANDS[num / 1000],
        HUNDREDS[num % 1000 / 100],
        TENS[num % 100 / 10],
        ONES[num % 10]
    )
}

fn main() {
    assert_eq!(int_to_roman(3), "III");
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV");
}
