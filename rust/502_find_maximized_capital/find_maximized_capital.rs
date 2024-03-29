/*
 * @Date: 2021-09-08 09:31:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-08 11:20:06
 */

struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    fn find_maximized_capital(mut k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut sorted_capital: Vec<(i32, usize)> = vec![];
        for i in 0..capital.len() {
            sorted_capital.push((capital[i], i));
        }
        sorted_capital.sort_unstable();
        sorted_capital.reverse();
        let mut res = w;
        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        loop {
            while let Some(&(c, i)) = sorted_capital.last() {
                if c <= res {
                    sorted_capital.pop();
                    queue.push(profits[i]);
                } else {
                    break;
                }
            }
            if let Some(max) = queue.pop() {
                res += max;
                k -= 1;
            } else {
                break;
            }
            if k == 0 {
                break;
            }
        }
        res
    }
}

fn main() {
    {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        let ans = 4;
        assert_eq!(
            Solution::find_maximized_capital(k, w, profits, capital),
            ans
        );
    }
    {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        let ans = 6;
        assert_eq!(
            Solution::find_maximized_capital(k, w, profits, capital),
            ans
        );
    }
}
