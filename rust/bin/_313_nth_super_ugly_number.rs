/*
 * @Date: 2021-08-09 11:24:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-09 11:55:18
 */

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        Self::nth_super_ugly_number_min_heap(n, primes)
        Self::nth_super_ugly_number_dp(n, primes)
    }
    pub fn nth_super_ugly_number_min_heap(n: i32, primes: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(1));
        let mut hs = HashSet::new();
        hs.insert(1);

        for _ in 1..n {
            let cur_ugly = heap.pop().unwrap().0;
            for p in primes.iter() {
                let new_ugly = p.checked_mul(cur_ugly).unwrap_or_default();
                if new_ugly == 0 {
                    continue;
                }
                if !hs.contains(&new_ugly) {
                    hs.insert(new_ugly);
                    heap.push(Reverse(new_ugly));
                }
            }
        }

        heap.peek().unwrap().0
    }
    pub fn nth_super_ugly_number_dp(n: i32, primes: Vec<i32>) -> i32 {
        let k = primes.len();
        let mut dp = Vec::with_capacity(n as usize);
        dp.push(1);
        let mut index = vec![0; k];

        for _ in 1..n as usize {
            let mut min = i32::MAX;
            for i in 0..k {
                min = min.min(primes[i] * dp[index[i]]);
            }
            dp.push(min);
            for i in 0..k {
                if primes[i] * dp[index[i]] == min {
                    index[i] += 1;
                }
            }
        }

        dp[n as usize - 1]
    }
}

fn main() {
    {
        let primes = vec![2, 7, 13, 19];
        let n = 12;
        let ans = 32;
        assert_eq!(
            Solution::nth_super_ugly_number_min_heap(n, primes.clone()),
            ans
        );
        assert_eq!(Solution::nth_super_ugly_number_dp(n, primes.clone()), ans);
    }
    {
        let primes = vec![2, 3, 5];
        let n = 1;
        let ans = 1;
        assert_eq!(
            Solution::nth_super_ugly_number_min_heap(n, primes.clone()),
            ans
        );
        assert_eq!(Solution::nth_super_ugly_number_dp(n, primes.clone()), ans);
    }
}
