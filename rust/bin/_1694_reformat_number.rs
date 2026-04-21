/*
 * @Date: 2022-10-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-01
 * @FilePath: /algorithm/1694_reformat_number/reformat_number.rs
 */

pub fn reformat_number(number: String) -> String {
    let mut digits: Vec<Vec<char>> = number
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|v| v.to_vec())
        .collect();

    if digits.last().unwrap().len() == 1 {
        let len = digits.len();
        let c = digits[len - 2].pop().unwrap();
        digits[len - 1].insert(0, c);
    }

    digits.join(&'-').into_iter().collect()
}

fn main() {
    assert_eq!(
        reformat_number(String::from("1-23-45 6")),
        String::from("123-456")
    );

    assert_eq!(
        reformat_number(String::from("123 4-567")),
        String::from("123-45-67")
    );

    assert_eq!(
        reformat_number(String::from("123 4-5678")),
        String::from("123-456-78")
    );

    assert_eq!(reformat_number(String::from("12")), String::from("12"));

    assert_eq!(
        reformat_number(String::from("--17-5 229 35-39475 ")),
        String::from("175-229-353-94-75")
    );
}
