/*
 * @Date: 2022-03-25 23:07:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-26 00:04:53
 * @FilePath: /algorithm/682_cal_points/cal_points.rs
 */

pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for op in ops.iter() {
        match op.as_str() {
            "+" => {
                let last = stack.pop().unwrap();
                let last_last = stack.pop().unwrap();
                let cur = last + last_last;
                stack.push(last_last);
                stack.push(last);
                stack.push(cur);
            }
            "D" => {
                let last = stack.pop().unwrap();
                let cur = last * 2;
                stack.push(last);
                stack.push(cur);
            }
            "C" => {
                stack.pop().unwrap();
            }
            _ => {
                let n = op.parse::<i32>().unwrap();
                stack.push(n);
            }
        }
    }
    stack.iter().sum()
}

fn main() {
    {
        let ops: Vec<String> = vec!["5", "2", "C", "D", "+"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        assert_eq!(cal_points(ops), 30);
    }

    {
        let ops: Vec<String> = vec!["5", "-2", "4", "C", "D", "9", "+", "+"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        assert_eq!(cal_points(ops), 27);
    }

    {
        let ops: Vec<String> = vec!["1"].iter().map(|&x| x.to_string()).collect();
        assert_eq!(cal_points(ops), 1);
    }
}
