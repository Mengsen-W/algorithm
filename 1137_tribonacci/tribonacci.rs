/*
 * @Date: 2021-08-08 11:40:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-08 13:45:27
 */

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1..=2 => 1,
            _ => {
                let q = vec![vec![1, 1, 1], vec![1, 0, 0], vec![0, 1, 0]];
                let res = Self::pow(q, n);
                res[0][2]
            }
        }
    }

    fn pow(mut a: Vec<Vec<i32>>, mut n: i32) -> Vec<Vec<i32>> {
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
    fn multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut c: Vec<Vec<i32>> = vec![vec![0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
            }
        }
        c
    }
}

fn main() {
    assert_eq!(Solution::tribonacci(4), 4);
    assert_eq!(Solution::tribonacci(25), 1389537);
}
