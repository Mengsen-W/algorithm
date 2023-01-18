/*
 * @Date: 2022-07-01
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-01
 * @FilePath: /algorithm/241_diff_ways_to_compute/diff_ways_to_compute.rs
 */

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    let n = expression.len();
    let mut res = vec![];
    if n == 0 {
        return res;
    }
    //将表达式依据运算符分为左右两边
    let chars = expression.as_bytes();
    for i in 0..n {
        match chars[i] {
            b'+' | b'-' | b'*' => {
                let left_res = diff_ways_to_compute(expression[..i].to_owned());
                let right_res = diff_ways_to_compute(expression[i + 1..].to_owned());
                for l in left_res.iter() {
                    for r in right_res.iter() {
                        match chars[i] {
                            b'+' => res.push(l + r),
                            b'-' => res.push(l - r),
                            _ => res.push(l * r),
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    //可能当前表达式是个纯数字
    if res.is_empty() {
        res.push(expression.parse::<i32>().unwrap());
    }
    return res;
}

fn main() {
    assert_eq!(diff_ways_to_compute(String::from("2-1-1")), vec![2, 0]);
    assert_eq!(
        diff_ways_to_compute(String::from("2*3-4*5")),
        vec![-34, -10, -14, -10, 10]
    );
}
