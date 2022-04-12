/*
 * @Date: 2022-04-12 09:11:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-12 09:30:07
 * @FilePath: /algorithm/806_number_of_lines/number_of_lines.rs
 */

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 1;
    let mut count = 0;
    for c in s.chars() {
        let width = widths[c as usize - 'a' as usize];
        if count + width > 100 {
            lines += 1;
            count = width;
        } else {
            count += width;
        }
    }
    vec![lines, count]
}

fn main() {
    assert_eq!(
        number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("abcdefghijklmnopqrstuvwxyz")
        ),
        vec![3, 60]
    );

    assert_eq!(
        number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("bbbcccdddaaa")
        ),
        vec![2, 4]
    );
}
