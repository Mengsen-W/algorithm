/*
 * @Date: 2023-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-27
 * @FilePath: /algorithm/rust/2309_greatest_letter/greatest_letter.rs
 */

pub fn greatest_letter(s: String) -> String {
    let (mut lower, mut upper) = (0, 0);
    s.chars().for_each(|x| {
        if x.is_lowercase() {
            lower |= 1 << (x as u32 - 'a' as u32);
        } else {
            upper |= 1 << (x as u32 - 'A' as u32);
        }
    });
    for i in (0..=25).rev() {
        if (lower & upper & (1 << i)) != 0 {
            return String::from(char::from_u32('A' as u32 + i).unwrap());
        }
    }
    String::new()
}

fn main() {
    {
        let s = "lEeTcOdE".to_string();
        let ans = "E".to_string();
        assert_eq!(greatest_letter(s), ans);
    }

    {
        let s = "arRAzFif".to_string();
        let ans = "R".to_string();
        assert_eq!(greatest_letter(s), ans);
    }

    {
        let s = "AbCdEfGhIjK".to_string();
        let ans = "".to_string();
        assert_eq!(greatest_letter(s), ans);
    }
}
