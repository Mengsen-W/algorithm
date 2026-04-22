/*
 * @Date: 2023-04-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-26
 * @FilePath: /algorithm/rust/1031_max_sum_two_no_overlap/max_sum_two_no_overlap.rs
 */

pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    // 分别求出前缀和后缀first和second的最大值
    let n: usize = nums.len();
    let first_len = first_len as usize;
    let second_len = second_len as usize;
    // 从前向后求出first,second
    let count_pre_max = |length: usize| -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; n];
        let mut pre_sum: i32 = nums[0..length].iter().sum();
        result[length - 1] = pre_sum;
        for index in length..n {
            pre_sum = pre_sum - nums[index - length] + nums[index];
            result[index] = std::cmp::max(pre_sum, result[index - 1]);
        }
        result
    };
    let pre_first_max = count_pre_max(first_len);
    let pre_second_max = count_pre_max(second_len);
    // 从后向前求出first,second
    let count_back_max = |length: usize| -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; n];
        let mut back_sum: i32 = nums[(n - length)..n].iter().sum();
        result[n - length] = back_sum;
        for index in (0..(n - length)).rev() {
            back_sum = back_sum - nums[index + length] + nums[index];
            result[index] = std::cmp::max(back_sum, result[index + 1]);
        }
        result
    };
    let back_first_max = count_back_max(first_len);
    let back_second_max = count_back_max(second_len);
    // 枚举
    let mut res: i32 = 0;
    // first在second前面
    for index in (first_len - 1)..(n - second_len) {
        res = std::cmp::max(res, pre_first_max[index] + back_second_max[index + 1]);
    }
    // second在first前面
    for index in (second_len - 1)..(n - first_len) {
        res = std::cmp::max(res, pre_second_max[index] + back_first_max[index + 1]);
    }
    res
}

fn main() {
    {
        let nums = vec![0, 6, 5, 2, 2, 5, 1, 9, 4];
        let first_len = 1;
        let second_len = 2;
        let ans = 20;
        assert_eq!(max_sum_two_no_overlap(nums, first_len, second_len), ans);
    }

    {
        let nums = vec![3, 8, 1, 3, 2, 1, 8, 9, 0];
        let first_len = 3;
        let second_len = 2;
        let ans = 29;
        assert_eq!(max_sum_two_no_overlap(nums, first_len, second_len), ans);
    }

    {
        let nums = vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
        let first_len = 4;
        let second_len = 3;
        let ans = 31;
        assert_eq!(max_sum_two_no_overlap(nums, first_len, second_len), ans);
    }
}
