/*
 * @Date: 2021-09-04 10:14:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-04 10:24:24
 */

struct Solution;

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn fib(n: i32) -> i32 {
        match n < 2 {
            true => return n,
            false => {
                let q = vec![vec![1, 1], vec![1, 0]];
                let res = Self::pow(q, n - 1);
                return res[0][0] as i32;
            }
        }
    }
    fn pow(mut a: Vec<Vec<i64>>, mut n: i32) -> Vec<Vec<i64>> {
        let mut ret = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        while n > 0 {
            if n & 1 == 1 {
                ret = Self::multiply(&ret, &a);
            }
            n >>= 1;
            a = Self::multiply(&a, &a);
        }
        ret
    }
    fn multiply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut c: Vec<Vec<i64>> = vec![vec![0; 3]; 3];
        for i in 0..2 {
            for j in 0..2 {
                c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j]) % Self::MOD;
            }
        }
        c
    }
}

fn main() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib(48), 807526948);
}
