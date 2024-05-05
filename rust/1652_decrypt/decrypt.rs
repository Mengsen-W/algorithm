/*
 * @Date: 2022-09-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-05
 * @FilePath: /algorithm/rust/1652_decrypt/decrypt.rs
 */

struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let s = if k > 0 { 1 } else { (n as i32 + k) as usize };
        let e = if k > 0 { k as usize + 1 } else { n };
        let mut sum = code[s..e].iter().sum();
        let mut ret = vec![0; n];

        for i in 0..n {
            ret[i] = sum;
            sum += code[(e + i) % n] - code[(s + i) % n];
        }

        ret
    }
}

fn main() {
    let tests = vec![
        (vec![5, 7, 1, 4], 3, vec![12, 10, 16, 13]),
        (vec![1, 2, 3, 4], 0, vec![0, 0, 0, 0]),
        (vec![2, 4, 9, 3], -2, vec![12, 5, 6, 13]),
    ];

    for (code, k, ans) in tests {
        assert_eq!(Solution::decrypt(code, k), ans);
    }
}
