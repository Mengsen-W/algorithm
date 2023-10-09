/*
 * @Date: 2023-10-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-09
 * @FilePath: /algorithm/rust/2578_split_num/split_num.rs
 */

struct Solution;
impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut s = num.to_string().into_bytes();
        s.sort_unstable();

        let mut ans = vec![0; 2];
        for (i, c) in s.iter().enumerate() {
            ans[i & 1] = ans[i & 1] * 10 + (c - b'0') as i32;
        }

        ans[0] + ans[1]
    }
}

fn main() {
    let tests = vec![(4325, 59), (687, 75)];

    for (num, ans) in tests {
        assert_eq!(Solution::split_num(num), ans);
    }
}
