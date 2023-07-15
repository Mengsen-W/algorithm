/*
 * @Date: 2023-07-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-15
 * @FilePath: /algorithm/rust/18_four_sum/four_sum.rs
 */

struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut nums = nums;
        let n = nums.len();
        let mut res = vec![];
        if n < 4 {
            return res;
        }
        nums.sort_unstable();
        for i in 0..n - 3 {
            if (&nums[i..i + 4]).iter().map(|x| *x as i64).sum::<i64>() > target as i64 {
                // 剪枝
                break;
            }
            if nums[i] as i64 + (&nums[n - 3..n]).iter().map(|x| *x as i64).sum::<i64>()
                < target as i64
            {
                // 剪枝
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                // 去重
                continue;
            }
            for j in i + 1..n - 2 {
                if nums[i] as i64 + (&nums[j..j + 3]).iter().map(|x| *x as i64).sum::<i64>()
                    > target as i64
                {
                    // 剪枝
                    break;
                }
                if nums[i] as i64
                    + nums[j] as i64
                    + (&nums[n - 2..n]).iter().map(|x| *x as i64).sum::<i64>()
                    < target as i64
                {
                    // 剪枝
                    continue;
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    // 去重
                    continue;
                }
                let (mut l, mut r) = (j + 1, n - 1);
                while l < r {
                    match (nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64)
                        .cmp(&(target as i64))
                    {
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
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
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 0, -1, 0, -2, 2],
            0,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        ),
        (vec![2, 2, 2, 2, 2], 8, vec![vec![2, 2, 2, 2]]),
    ];

    for (nums, target, ret) in tests {
        assert_eq!(Solution::four_sum(nums, target), ret);
    }
}
