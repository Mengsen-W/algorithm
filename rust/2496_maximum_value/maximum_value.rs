/*
 * @Date: 2023-06-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-23
 * @FilePath: /algorithm/rust/2496_maximum_value/maximum_value.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter()
            .map(|s| match s.parse::<i32>() {
                Ok(x) => x,
                Err(_) => s.len() as i32,
            })
            .max()
            .unwrap()
    }
}

fn main() {
    {
        let strs = ["alic3", "bob", "3", "4", "00000"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans = 5;
        assert_eq!(Solution::maximum_value(strs), ans);
    }

    {
        let strs = ["1", "01", "001", "0001"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans = 1;
        assert_eq!(Solution::maximum_value(strs), ans);
    }
}
