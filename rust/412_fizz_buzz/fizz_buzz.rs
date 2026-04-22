/*
 * @Date: 2021-10-13 08:45:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-13 09:14:22
 */

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for i in 1..=n {
            match i {
                x if x % 15 == 0 => result.push("FizzBuzz".to_string()),
                x if x % 3 == 0 => result.push("Fizz".to_string()),
                x if x % 5 == 0 => result.push("Buzz".to_string()),
                _ => result.push(i.to_string()),
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::fizz_buzz(15),
        [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
    )
}
