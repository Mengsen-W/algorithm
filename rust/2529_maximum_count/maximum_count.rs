/*
 * @Date: 2024-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-09
 * @FilePath: /algorithm/rust/2529_maximum_count/maximum_count.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        fn lower_bound(nums: &Vec<i32>, val: i32) -> usize {
            let mut l = 0;
            let mut r = nums.len();
            while l < r {
                let m = (l + r) / 2;
                if nums[m] >= val {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            l
        }

        let pos1 = lower_bound(&nums, 0);
        let pos2 = lower_bound(&nums, 1);
        pos1.max(nums.len() - pos2) as i32
    }
}

fn main() {
    let tests = vec![
        (vec![-2, -1, -1, 1, 2, 3], 3),
        (vec![-3, -2, -1, 0, 0, 1, 2], 3),
        (vec![5, 20, 66, 1314], 4),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::maximum_count(nums), ans);
    }
}
