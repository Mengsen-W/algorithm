/*
 * @Date: 2021-08-31 14:23:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-31 14:39:20
 */

struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![0; n as usize];
        for ve in &bookings {
            let l: usize = ve[0] as usize;
            let r: usize = ve[1] as usize;
            let siz: i32 = ve[2];
            ret[l - 1] += siz;
            if r != n as usize {
                ret[r] -= siz;
            }
        }
        for i in 1..n as usize {
            ret[i] += ret[i - 1];
        }
        ret
    }
}

fn main() {
    {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        let n = 5;
        let answer = vec![10, 55, 45, 25, 25];
        assert_eq!(Solution::corp_flight_bookings(bookings, n), answer);
    }
    {
        let bookings = vec![vec![1, 2, 10], vec![2, 2, 15]];
        let n = 2;
        let answer = vec![10, 25];
        assert_eq!(Solution::corp_flight_bookings(bookings, n), answer);
    }
}
