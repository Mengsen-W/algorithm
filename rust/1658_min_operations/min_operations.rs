/*
 * @Date: 2023-01-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-07
 * @FilePath: /algorithm/1658_min_operations/min_operations.rs
 */

pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let n = nums.len();
    let target = nums.iter().sum::<i32>() - x;

    let (mut l, mut r) = (0, 0);
    let (mut sum, mut max) = (0, -1);
    while l <= r && r <= n {
        if sum == target {
            max = max.max((r - l) as i32);
        }

        if sum <= target {
            sum += nums.get(r).unwrap_or(&0);
            r += 1;
        } else {
            sum -= nums.get(l).unwrap_or(&0);
            l += 1;
        }
    }

    if max != -1 {
        return n as i32 - max;
    }
    -1
}

fn main() {
    {
        let nums = vec![1, 1, 4, 2, 3];
        let x = 5;
        let ans = 2;
        assert_eq!(min_operations(nums, x), ans);
    }

    {
        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;
        let ans = -1;
        assert_eq!(min_operations(nums, x), ans);
    }

    {
        let nums = vec![3, 2, 20, 1, 1, 3];
        let x = 10;
        let ans = 5;
        assert_eq!(min_operations(nums, x), ans);
    }
}
