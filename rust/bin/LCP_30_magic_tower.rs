/*
 * @Date: 2024-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-06
 * @FilePath: /algorithm/rust/LCP_30_magic_tower/magic_tower.rs
 */

struct Solution;

impl Solution {
    pub fn magic_tower(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        if nums.iter().map(|&x| x as i64).sum::<i64>() < 0 {
            return -1;
        }
        let mut ans = 0;
        let mut hp = 1i64;
        let mut h = BinaryHeap::new(); // 最大堆
        for &x in &nums {
            if x < 0 {
                h.push(-x);
            }
            hp += x as i64;
            if hp < 1 {
                hp += h.pop().unwrap() as i64;
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150], 1),
        (vec![-200, -300, 400, 0], -1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::magic_tower(nums), ans);
    }
}
