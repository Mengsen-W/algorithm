/*
 * @Date: 2022-01-07 01:08:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-07 01:25:02
 */

pub fn max_depth(s: String) -> i32 {
    s.chars()
        .fold((0, 0), |(mx, cnt), c| match c {
            '(' => (mx.max(cnt + 1), cnt + 1),
            ')' => (mx, cnt - 1),
            _ => (mx, cnt),
        })
        .0
}

fn main() {
    assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    assert_eq!(max_depth("(1)+((2))+(((3)))".to_string()), 3);
    assert_eq!(max_depth("1+(2*3)/(2-1)".to_string()), 1);
    assert_eq!(max_depth("1".to_string()), 0);
}
