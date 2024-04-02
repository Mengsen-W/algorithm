/*
 * @Date: 2024-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-01
 * @FilePath: /algorithm/rust/2810_final_string/final_string.rs
 */

struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut q = Vec::<char>::new();
        let mut head = false;
        for ch in s.chars() {
            if ch != 'i' {
                if head {
                    q.insert(0, ch);
                } else {
                    q.push(ch);
                }
            } else {
                head = !head;
            }
        }
        let ans: String = if head {
            q.iter().rev().collect()
        } else {
            q.iter().collect()
        };
        ans
    }
}

fn main() {
    let tests = vec![("string", "rtsng"), ("poiinter", "ponter")];

    for (s, ans) in tests {
        assert_eq!(Solution::final_string(s.to_string()), ans.to_string());
    }
}
