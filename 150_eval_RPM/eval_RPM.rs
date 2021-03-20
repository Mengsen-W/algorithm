/*
 * @Date: 2021-03-20 09:03:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-20 09:09:59
 * @FilePath: \algorithm\150_eval_RPM\eval_RPM.rs
 * @Description: file content
 */

fn eval_rpn(mut tokens: Vec<&str>) -> i32 {
    let mut v = Vec::with_capacity(10);
    tokens.drain(..).for_each(|x| match x {
        "+" => {
            let c = v.pop().unwrap();
            *v.last_mut().unwrap() += c
        }
        "-" => {
            let c = v.pop().unwrap();
            *v.last_mut().unwrap() -= c
        }
        "*" => {
            let c = v.pop().unwrap();
            *v.last_mut().unwrap() *= c
        }
        "/" => {
            let c = v.pop().unwrap();
            *v.last_mut().unwrap() /= c
        }
        x => v.push(x.parse().unwrap()),
    });
    v.last().copied().unwrap()
}

fn main() {
    assert_eq!(eval_rpn(vec!["2", "1", "+", "3", "*"]), 9);
    assert_eq!(eval_rpn(vec!["4", "13", "5", "/", "+"]), 6);
    assert_eq!(
        eval_rpn(vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]),
        22
    );
}
