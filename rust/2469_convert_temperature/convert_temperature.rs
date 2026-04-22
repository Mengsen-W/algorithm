/*
 * @Date: 2023-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-21
 * @FilePath: /algorithm/rust/2469_convert_temperature/convert_temperature.rs
 */

pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![celsius + 273.15, celsius * 1.80 + 32.00]
}

fn main() {
    assert_eq!(convert_temperature(36.50), vec![309.65000, 97.70000]);
    assert_eq!(convert_temperature(122.11), vec![395.26000, 251.79800]);
}
