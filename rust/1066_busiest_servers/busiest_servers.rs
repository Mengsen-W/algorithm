/*
 * @Date: 2022-03-29 23:53:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-29 23:53:59
 * @FilePath: /algorithm/1066_busiest_servers/busiest_servers.rs
 */

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, BinaryHeap};

        let mut freq = vec![0; k as usize];
        let mut max = 0;
        let mut free = BTreeSet::<i32>::new();
        let mut busy = BinaryHeap::<(i32, i32)>::new(); // (avail(x), x)
        for i in 0..k {
            free.insert(i as i32);
        }

        for i in 0..arrival.len() {
            while !busy.is_empty() && -busy.peek().unwrap().0 <= arrival[i] {
                free.insert(busy.pop().unwrap().1);
            }
            if free.is_empty() {
                continue;
            }
            let pos = i % k as usize;
            if free.contains(&(pos as i32)) {
                free.remove(&(pos as i32));
                let avail = arrival[i] + load[i];
                busy.push((-avail, pos as i32));
                freq[pos as usize] += 1;
                if freq[pos] > max {
                    max = freq[pos];
                }
            } else {
                let mut next = free.range(pos as i32..).next();
                if next.is_none() {
                    next = free.range(0..).next();
                }
                let x = *next.unwrap();
                free.remove(&x);
                let avail = arrival[i] + load[i];
                busy.push((-avail, x));
                freq[x as usize] += 1;
                if freq[x as usize] > max {
                    max = freq[x as usize];
                }
            }
        }

        let mut ans = vec![];
        for (i, &x) in freq.iter().enumerate() {
            if x == max {
                ans.push(i as i32)
            }
        }
        ans
    }
}

fn main() {
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
        vec![1]
    );

    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
        vec![0]
    );

    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
        vec![0, 1, 2]
    );

    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4, 8, 9, 10], vec![5, 2, 10, 3, 1, 2, 2]),
        vec![1]
    );

    assert_eq!(Solution::busiest_servers(1, vec![1], vec![1]), vec![0]);
}
