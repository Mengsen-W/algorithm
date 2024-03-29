/*
 * @Date: 2021-10-30 01:13:07
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-16
 */

struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut all = 0;
        for num in &nums {
            all ^= num;
        }
        let diff = all & (-all);
        let mut x = 0;
        for num in &nums {
            if (diff & num) != 0 {
                x ^= num;
            }
        }
        vec![x, x ^ all]
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1, 3, 2, 5], vec![3, 5]),
        (vec![-1, 0], vec![-1, 0]),
        (vec![0, 1], vec![1, 0]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::single_number(nums), ans);
    }
}
