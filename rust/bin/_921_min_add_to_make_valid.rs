/*
 * @Date: 2022-10-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-04
 * @FilePath: /algorithm/921_min_add_to_make_valid/min_add_to_make_valid.rs
 */

pub fn min_add_to_make_valid(s: String) -> i32 {
    s.chars()
        .fold([0, 0], |mut array, c| {
            if c == '(' {
                array[0] += 1;
            } else if array[0] > 0 {
                array[0] -= 1;
            } else {
                array[1] += 1;
            }

            array
        })
        .iter()
        .sum()
}

fn main() {
    assert_eq!(min_add_to_make_valid(String::from("())")), 1);
    assert_eq!(min_add_to_make_valid(String::from("(((")), 3);
}
