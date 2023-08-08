/*
 * @Date: 2023-08-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-08
 * @FilePath: /algorithm/rust/1749_max_absolute_sum/max_absolute_sum.rs
 */

struct Solution;
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0, 0), |(ma, mi, res), &x| {
                let ma = (ma + x).max(x);
                let mi = (mi + x).min(x);
                (ma, mi, res.max(ma).max(-mi))
            })
            .2
    }
}

fn main() {
    let tests = vec![(vec![1, -3, 2, 3, -4], 5), (vec![2, -5, 1, -4, 3, -2], 8)];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_absolute_sum(nums), ans);
    }
}
