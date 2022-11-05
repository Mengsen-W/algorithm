/*
 * @Date: 2022-11-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-05
 * @FilePath: /algorithm/1106_parse_bool_expr/parse_bool_expr.rs
 */

pub fn parse_bool_expr(expression: String) -> bool {
    use std::str::Chars;
    fn parse(it: &mut Chars<'_>) -> bool {
        match it.next() {
            Some('t') => true,
            Some('f') => false,
            Some('!') => parse_not(it),
            Some('&') => parse_and_or(it, true, |a, b| a & b),
            Some('|') => parse_and_or(it, false, |a, b| a | b),
            _ => unreachable!(),
        }
    }

    fn parse_not(it: &mut Chars<'_>) -> bool {
        it.next();

        let ans = parse(it);

        it.next();

        !ans
    }

    fn parse_and_or<F>(it: &mut Chars<'_>, init_val: bool, func: F) -> bool
    where
        F: Fn(bool, bool) -> bool,
    {
        let mut ans = init_val;

        it.next();

        loop {
            ans = func(ans, parse(it));

            match it.next() {
                Some(',') => continue,
                Some(')') => break,
                _ => unreachable!(),
            }
        }

        ans
    }
    parse(&mut expression.chars())
}

fn main() {
    assert!(parse_bool_expr(String::from("!(f)")));
    assert!(parse_bool_expr(String::from("|(f,t)")));
    assert!(!parse_bool_expr(String::from("&(t,f)")));
    assert!(!parse_bool_expr(String::from("|(&(t,f,t),!(t))")));
}
