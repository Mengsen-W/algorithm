/*
 * @Date: 2023-07-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-09
 * @FilePath: /algorithm/rust/15_three_sum/three_sum.rs
 */

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut nums = nums;
        let n = nums.len();
        let mut res = vec![];
        if n < 3 {
            return res;
        }
        nums.sort_unstable();
        for i in 0..n - 2 {
            if (&nums[i..i + 3]).iter().sum::<i32>() > 0 {
                // 剪枝
                break;
            }
            if nums[i] + (&nums[n - 2..n]).iter().sum::<i32>() < 0 {
                // 剪枝
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                // 去重
                continue;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            // 去重
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            // 去重
                            r -= 1;
                        }
                    }
                    Ordering::Greater => r -= 1,
                    Ordering::Less => l += 1,
                }
            }
        }
        res
    }
}

fn main() {
    let test_map = vec![
        (
            vec![-1, 0, 1, 2, -1, 4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        ),
        (vec![0, 1, 1], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
    ];

    for case in test_map {
        assert_eq!(Solution::three_sum(case.0), case.1);
    }
}
