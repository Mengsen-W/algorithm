/*
 * @Date: 2021-04-15 09:17:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-15 09:34:57
 */

fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        return nums[0];
    }
    let rob_help = |nums: &Vec<i32>, start: usize, end: usize| -> i32 {
        if start > end {
            return 0;
        }
        let mut dp_i = 0;
        let mut dp_1 = 0;
        let mut dp_2 = 0;

        for i in start..=end {
            dp_i = std::cmp::max(dp_1, dp_2 + nums[i]);
            dp_2 = dp_1;
            dp_1 = dp_i;
        }
        return dp_i;
    };
    std::cmp::max(
        rob_help(&nums, 0, nums.len() - 2),
        rob_help(&nums, 1, nums.len() - 1),
    )
}

fn main() {
    {
        let nums = vec![2, 3, 2];
        assert_eq!(rob(nums), 3);
    }
    {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
    }
    {
        let nums = vec![0];
        assert_eq!(rob(nums), 0);
    }
}
