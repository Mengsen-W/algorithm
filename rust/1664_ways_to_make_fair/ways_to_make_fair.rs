/*
 * @Date: 2023-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-28
 * @FilePath: /algorithm/rust/1664_ways_to_make_fair/ways_to_make_fair.rs
 */

pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let (mut odd1, mut even1, mut odd2, mut even2) = (0, 0, 0, 0);
    for i in 0..nums.len() {
        if (i & 1) != 0 {
            odd2 += nums[i];
        } else {
            even2 += nums[i];
        }
    }
    let mut res = 0;
    for i in 0..nums.len() {
        if (i & 1) != 0 {
            odd2 -= nums[i];
        } else {
            even2 -= nums[i];
        }
        if odd1 + even2 == odd2 + even1 {
            res += 1;
        }
        if (i & 1) != 0 {
            odd1 += nums[i];
        } else {
            even1 += nums[i];
        }
    }
    res
}

fn main() {
    {
        let nums = vec![2, 1, 6, 4];
        let ans = 1;
        assert_eq!(ways_to_make_fair(nums), ans);
    }

    {
        let nums = vec![1, 1, 1];
        let ans = 3;
        assert_eq!(ways_to_make_fair(nums), ans);
    }

    {
        let nums = vec![1, 2, 3];
        let ans = 0;
        assert_eq!(ways_to_make_fair(nums), ans);
    }
}
