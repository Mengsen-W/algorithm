/*
 * @Date: 2023-08-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-18
 * @FilePath: /algorithm/rust/1388_max_size_slices/max_size_slices.rs
 */

struct Solution;

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        fn calc(s: &[i32]) -> i32 {
            let mut pre = vec![0; s.len() + 2];
            for _ in 0..s.len() / 3 + 1 {
                let mut cur = vec![0; s.len() + 2];
                for i in 2..pre.len() {
                    cur[i] = cur[i - 1].max(pre[i - 2] + s[i - 2]);
                }
                pre = cur;
            }
            pre[s.len() + 1]
        }
        calc(&slices[0..slices.len() - 1]).max(calc(&slices[1..]))
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 4, 5, 6], 10), (vec![8, 9, 8, 6, 1, 1], 16)];

    for (slices, ans) in tests {
        assert_eq!(Solution::max_size_slices(slices), ans)
    }
}
