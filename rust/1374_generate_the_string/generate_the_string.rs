/*
 * @Date: 2022-08-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-01
 * @FilePath: /algorithm/1374_generate_the_string/generate_the_string.rs
 */

pub fn generate_the_string(n: i32) -> String {
    format!(
        "b{}",
        (vec!["a", "b"][n as usize % 2..(n as usize % 2) + 1].join("")).repeat(n as usize - 1)
    )
}

fn main() {
    println!("{}", generate_the_string(4));
    println!("{}", generate_the_string(2));
}
