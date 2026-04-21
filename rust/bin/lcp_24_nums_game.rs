/*
 * @Date: 2024-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-01
 * @FilePath: /algorithm/rust/lcp_24_nums_game/nums_game.rs
 */

struct Solution;

impl Solution {
    pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        const MOD: i64 = 1_000_000_007;
        let mut ans = vec![0; nums.len()];
        let mut left = BinaryHeap::new(); // 维护较小的一半，大根堆
        let mut right = BinaryHeap::new(); // 维护较大的一半，小根堆
        let mut left_sum = 0i64;
        let mut right_sum = 0i64;
        for (i, &num) in nums.iter().enumerate() {
            let mut b = num - i as i32;
            if i % 2 == 0 {
                // 前缀长度是奇数
                if let Some(&top) = left.peek() {
                    if b < top {
                        left_sum -= (top - b) as i64;
                        left.push(b);
                        b = left.pop().unwrap();
                    }
                }
                right_sum += b as i64;
                right.push(Reverse(b));
                ans[i] = ((right_sum - right.peek().unwrap().0 as i64 - left_sum) % MOD) as i32;
            } else {
                // 前缀长度是偶数
                let top = right.peek().unwrap().0;
                if b > top {
                    right_sum += (b - top) as i64;
                    right.push(Reverse(b));
                    b = right.pop().unwrap().0;
                }
                left_sum += b as i64;
                left.push(b);
                ans[i] = ((right_sum - left_sum) % MOD) as i32;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![3, 4, 5, 1, 6, 7], vec![0, 0, 0, 5, 6, 7]),
        (vec![1, 2, 3, 4, 5], vec![0, 0, 0, 0, 0]),
        (vec![1, 1, 1, 2, 3, 4], vec![0, 1, 2, 3, 3, 3]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::nums_game(nums), ans);
    }
}
