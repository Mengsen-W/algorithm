/*
 * @Date: 2023-04-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-28
 * @FilePath: /algorithm/rust/1017_base_neg2/base_neg2.rs
 */

struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut val = 0x55555555 ^ (0x55555555 - n) as u32;
        if val == 0 {
            return String::from("0");
        }
        let mut res = Vec::new();
        while val != 0 {
            res.push(char::from_u32('0' as u32 + (val & 1)).unwrap());
            val >>= 1;
        }
        res.reverse();
        res.into_iter().collect()
    }
}

fn main() {
    let tests = vec![(2, "110"), (3, "111"), (4, "100")];

    for (n, ans) in tests {
        assert_eq!(Solution::base_neg2(n), ans.to_string());
    }
}
