/*
 * @Date: 2021-08-18 08:36:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-18 21:29:15
 */

struct Solution;

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn check_record(n: i32) -> i32 {
        let mat: Vec<Vec<i64>> = vec![
            vec![1, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 0, 0],
            vec![1, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 0, 0],
        ];
        let res = Self::pow(mat, n);
        let sum: i64 = res.first().unwrap().iter().sum();
        (sum % Self::MOD) as i32
    }
    fn pow(mut mat: Vec<Vec<i64>>, mut n: i32) -> Vec<Vec<i64>> {
        let mut ret: Vec<Vec<i64>> = vec![vec![1, 0, 0, 0, 0, 0]];
        while n > 0 {
            if (n & 1) == 1 {
                ret = Self::multiply(&ret, &mat);
            }
            n >>= 1;
            mat = Self::multiply(&mat, &mat)
        }
        ret
    }

    fn multiply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let rows = a.len();
        let columns = b[0].len();
        let temp = b.len();
        let mut c: Vec<Vec<i64>> = vec![vec![0; columns]; rows];
        for i in 0..rows {
            for j in 0..columns {
                for k in 0..temp {
                    c[i][j] += a[i][k] * b[k][j];
                    c[i][j] %= Self::MOD;
                }
            }
        }
        c
    }
}

fn main() {
    assert_eq!(Solution::check_record(2), 8);
    assert_eq!(Solution::check_record(1), 3);
    assert_eq!(Solution::check_record(10101), 183236316);
}
