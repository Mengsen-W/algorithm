/*
 * @Date: 2023-07-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-23
 * @FilePath: /algorithm/rust/42_trap/trap.rs
 */

struct Solution;
impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            if height[l] < height[r] {
                if height[l + 1] < height[l] {
                    res += height[l] - height[l + 1];
                    height[l + 1] = height[l];
                }
                l += 1;
            } else {
                if height[r - 1] < height[r] {
                    res += height[r] - height[r - 1];
                    height[r - 1] = height[r];
                }
                r -= 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];

    for (height, ans) in tests {
        assert_eq!(Solution::trap(height), ans);
    }
}
