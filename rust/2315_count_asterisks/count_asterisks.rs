/*
 * @Date: 2023-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-29
 * @FilePath: /algorithm/rust/2315_count_asterisks/count_asterisks.rs
 */

pub fn count_asterisks(s: String) -> i32 {
    s.chars()
        .fold((true, 0), |(mut valid, mut res), c| {
            if c == '|' {
                valid = !valid;
            } else if c == '*' && valid {
                res += 1;
            }
            (valid, res)
        })
        .1
}

fn main() {
    {
        let s = String::from("l|*e*et|c**o|*de|");
        let ans = 2;
        assert_eq!(count_asterisks(s), ans);
    }

    {
        let s = String::from("iamprogrammer");
        let ans = 0;
        assert_eq!(count_asterisks(s), ans);
    }

    {
        let s = String::from("yo|uar|e**|b|e***au|tifu|l");
        let ans = 5;
        assert_eq!(count_asterisks(s), ans);
    }
}
