/*
 * @Date: 2022-08-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-10
 * @FilePath: /algorithm/640_solve_equation/solve_equation.rs
 */

struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let equation = equation.replace("-", "+-");
        let mut exprs = equation.split("=");
        let (a, b) = Self::parse(exprs.next().unwrap());
        let (c, d) = Self::parse(exprs.next().unwrap());

        if a == c {
            if b == d {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            format!("x={}", (d - b) / (a - c))
        }
    }

    fn parse(expr: &str) -> (i32, i32) {
        let mut a = 0;
        let mut b = 0;

        for item in expr.split("+") {
            if item == "x" {
                a += 1;
            } else if item == "-x" {
                a -= 1;
            } else if item.ends_with("x") {
                a += item[..item.len() - 1].parse::<i32>().unwrap();
            } else if !item.is_empty() {
                b += item.parse::<i32>().unwrap();
            }
        }

        (a, b)
    }
}

fn main() {
    assert_eq!(
        Solution::solve_equation(String::from("x+5-3+x=6+x-2")),
        String::from("x=2")
    );
    assert_eq!(
        Solution::solve_equation(String::from("x=x")),
        String::from("Infinite solutions")
    );
    assert_eq!(
        Solution::solve_equation(String::from("2x=x")),
        String::from("x=0")
    );
}
