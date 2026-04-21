/*
 * @Date: 2021-03-12 09:02:29
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-31
 * @FilePath: /algorithm/rust/331_is_valid_serialization/is_valid_serialization.rs
 */

struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = Vec::new();
        let preorder: Vec<&str> = preorder.split_terminator(",").collect();
        for s in preorder.into_iter() {
            if s != "#" {
                stack.push(s);
                continue;
            }
            while !stack.is_empty() && stack[stack.len() - 1] == "#" {
                stack.pop();
                if let Some(x) = stack.pop() {
                    if x == "#" {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            stack.push("#");
        }
        stack.len() == 1 && stack[0] == "#"
    }
}

fn main() {
    assert!(Solution::is_valid_serialization(
        "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()
    ));
    assert!(!Solution::is_valid_serialization("1, #".to_string()));
    assert!(!Solution::is_valid_serialization("9, #, #, 1".to_string()));
    assert!(Solution::is_valid_serialization("9,#,92,#,#".to_string()));
}
