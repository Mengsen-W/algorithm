/*
 * @Date: 2021-07-20 14:04:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:12:20
 */

fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let (mut l, mut res, mut cost) = (0, 1, 0);
    let n = nums.len();
    for r in 1..n {
        cost += (nums[r] - nums[r - 1]) * ((r - l) as i32);
        while cost > k {
            cost -= nums[r] - nums[l];
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as i32
}

fn main() {
    {
        let nums = vec![1, 2, 4];
        let k = 5;
        let ans = 3;
        assert_eq!(max_frequency(nums, k), ans);
    }
    {
        let nums = vec![1, 4, 8, 13];
        let k = 5;
        let ans = 2;
        assert_eq!(max_frequency(nums, k), ans);
    }
}
