/*
 * @Date: 2022-09-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-24
 * @FilePath: /algorithm/1652_decrypt/decrypt.rs
 */

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

fn main() {
    {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let ans = vec![12, 10, 16, 13];
        assert_eq!(decrypt(code, k), ans);
    }

    {
        let code = vec![1, 2, 3, 4];
        let k = 0;
        let ans = vec![0, 0, 0, 0];
        assert_eq!(decrypt(code, k), ans);
    }

    {
        let code = vec![2, 4, 9, 3];
        let k = -2;
        let ans = vec![12, 5, 6, 13];
        assert_eq!(decrypt(code, k), ans);
    }
}
