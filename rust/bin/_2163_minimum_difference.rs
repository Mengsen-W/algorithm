struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n3 = nums.len();
        let n = n3 / 3;
        let mut part1 = vec![0i64; n + 1];
        let mut sum = 0i64;
        // 大根堆
        let mut ql = BinaryHeap::new();
        for i in 0..n {
            sum += nums[i] as i64;
            ql.push(nums[i]);
        }
        part1[0] = sum;
        for i in n..2 * n {
            sum += nums[i] as i64;
            ql.push(nums[i]);
            sum -= ql.pop().unwrap() as i64;
            part1[i - (n - 1)] = sum;
        }

        let mut part2 = 0i64;
        // 小根堆（用Reverse模拟）
        let mut qr = BinaryHeap::new();
        for i in (2 * n..3 * n).rev() {
            part2 += nums[i] as i64;
            qr.push(Reverse(nums[i]));
        }
        let mut ans = part1[n] - part2;
        for i in (n..2 * n).rev() {
            part2 += nums[i] as i64;
            qr.push(Reverse(nums[i]));
            if let Some(Reverse(val)) = qr.pop() {
                part2 -= val as i64;
            }
            ans = ans.min(part1[i - n] - part2);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![3, 1, 2], -1), (vec![7, 9, 5, 8, 1, 3], 1)];

    for (nums, expected) in tests {
        assert_eq!(Solution::minimum_difference(nums), expected);
    }
}
