/*
 * @Date: 2022-10-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-07
 * @FilePath: /algorithm/1800_max_ascending_sum/max_ascending_sum.rs
 */

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut tmp = nums[0];
    for i in nums.windows(2) {
        tmp = if i[1] > i[0] { tmp + i[1] } else { i[1] };
        res = std::cmp::max(res, tmp);
    }
    res
}

fn main() {
    {
        let nums = vec![10, 20, 30, 5, 10, 50];
        assert_eq!(max_ascending_sum(nums), 65);
    }

    {
        let nums = vec![10, 20, 30, 40, 50];
        assert_eq!(max_ascending_sum(nums), 150);
    }

    {
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        assert_eq!(max_ascending_sum(nums), 33);
    }

    {
        let nums = vec![100, 10, 1];
        assert_eq!(max_ascending_sum(nums), 100);
    }
}
