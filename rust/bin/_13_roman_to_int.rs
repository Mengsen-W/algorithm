/*
 * @Date: 2021-05-15 14:06:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-15 14:14:29
 */

fn roman_to_int(s: String) -> i32 {
    s.bytes()
        .rev()
        .fold((0, 0), |res, cur| {
            let n = match cur {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => -9999,
            };
            (if n < res.1 { res.0 - n } else { res.0 + n }, n)
        })
        .0
}

fn main() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("IV".to_string()), 4);
    assert_eq!(roman_to_int("IX".to_string()), 9);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}
