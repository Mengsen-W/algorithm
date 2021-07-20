/*
 * @Date: 2021-07-20 14:13:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:22:10
 */

fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    nums.sort();
    for i in 0..n {
        res = res.max(nums[i] + nums[n - i - 1]);
    }
    res
}

fn main() {
    {
        let nums = vec![3, 5, 2, 3];
        let ans = 7;
        assert_eq!(min_pair_sum(nums), ans);
    }
    {
        let nums = vec![3, 5, 4, 2, 4, 6];
        let ans = 8;
        assert_eq!(min_pair_sum(nums), ans);
    }
}
