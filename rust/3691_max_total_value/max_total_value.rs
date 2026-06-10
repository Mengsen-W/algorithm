struct Solution;

use std::collections::BinaryHeap;

struct SegTree {
    maxv: Vec<i32>,
    minv: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(nums: &Vec<i32>) -> Self {
        let n = nums.len();
        let mut seg = SegTree {
            maxv: vec![0; n * 4],
            minv: vec![0; n * 4],
            n,
        };
        seg.build(1, 0, n - 1, nums);
        seg
    }

    fn build(&mut self, node: usize, l: usize, r: usize, nums: &Vec<i32>) {
        if l == r {
            self.maxv[node] = nums[l];
            self.minv[node] = nums[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(node * 2, l, m, nums);
        self.build(node * 2 + 1, m + 1, r, nums);
        self.maxv[node] = self.maxv[node * 2].max(self.maxv[node * 2 + 1]);
        self.minv[node] = self.minv[node * 2].min(self.minv[node * 2 + 1]);
    }

    fn query_max(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql <= l && r <= qr {
            return self.maxv[node];
        }
        let m = (l + r) / 2;
        let mut res = i32::MIN;
        if ql <= m {
            res = res.max(self.query_max(node * 2, l, m, ql, qr));
        }
        if qr > m {
            res = res.max(self.query_max(node * 2 + 1, m + 1, r, ql, qr));
        }
        res
    }

    fn query_min(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql <= l && r <= qr {
            return self.minv[node];
        }
        let m = (l + r) / 2;
        let mut res = i32::MAX;
        if ql <= m {
            res = res.min(self.query_min(node * 2, l, m, ql, qr));
        }
        if qr > m {
            res = res.min(self.query_min(node * 2 + 1, m + 1, r, ql, qr));
        }
        res
    }
}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let seg = SegTree::new(&nums);
        let mut heap = BinaryHeap::new();
        for l in 0..n {
            let val = seg.query_max(1, 0, n - 1, l, n - 1) - seg.query_min(1, 0, n - 1, l, n - 1);
            heap.push((val, l, n - 1));
        }
        let mut ans: i64 = 0;
        let mut k = k as usize;
        while k > 0 {
            if let Some((val, l, r)) = heap.pop() {
                ans += val as i64;
                if r > l {
                    let new_val =
                        seg.query_max(1, 0, n - 1, l, r - 1) - seg.query_min(1, 0, n - 1, l, r - 1);
                    heap.push((new_val, l, r - 1));
                }
            }
            k -= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 2], 2, 4), (vec![4, 2, 5, 1], 3, 12)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_total_value(nums, k), expected);
    }
}
