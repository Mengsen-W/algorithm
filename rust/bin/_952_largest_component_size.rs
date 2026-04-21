/*
 * @Date: 2022-07-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-30
 * @FilePath: /algorithm/952_largest_component_size/largest_component_size.rs
 */

struct Solution;

use std::collections::HashMap;
pub struct UnionFind {
    id: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut id = vec![0; n];
        let mut sizes = vec![0; n];
        for i in 0..n {
            id[i] = i;
            sizes[i] = 1;
        }
        Self { id, sizes }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.id[x] != x {
            self.id[x] = self.find(self.id[x]);
        }
        self.id[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> usize {
        let (x, y) = (self.find(a), self.find(b));
        if x != y {
            self.id[x] = y;
            self.sizes[y] += self.sizes[x];
        }
        self.sizes[y]
    }
}

impl Solution {
    pub fn breakdown(a: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut n = a;
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                // 重复的只添加一次
                res.push(i);
                while n % i == 0 {
                    n /= i;
                }
            }
            i += 1;
        }

        if n != 1 {
            res.push(n);
        }
        res
    }

    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // 质因数对应数字的索引
        let mut m = HashMap::new();
        for (i, a) in nums.iter().enumerate() {
            for b in Self::breakdown(*a) {
                let e = m.entry(b).or_insert_with(Vec::new);
                e.push(i)
            }
        }

        let mut res = 1;
        let mut uf = UnionFind::new(n);
        for values in m.into_values() {
            for i in 1..values.len() {
                let size = uf.union(values[i - 1], values[i]);
                res = res.max(size);
            }
        }
        res as i32
    }
}

fn main() {
    {
        let nums = vec![4, 6, 15, 35];
        let ans = 4;
        assert_eq!(Solution::largest_component_size(nums), ans);
    }
    {
        let nums = vec![20, 50, 9, 63];
        let ans = 2;
        assert_eq!(Solution::largest_component_size(nums), ans);
    }
    {
        let nums = vec![2, 3, 6, 7, 4, 12, 21, 39];
        let ans = 8;
        assert_eq!(Solution::largest_component_size(nums), ans);
    }
}
