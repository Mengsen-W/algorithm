/*
 * @Date: 2022-12-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-11
 * @FilePath: /algorithm/1827_min_operations/min_operations.rs
 */

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let (mut pre, mut res) = (nums[0] - 1, 0);
    for num in nums {
        pre = num.max(pre + 1);
        res += pre - num;
    }
    res
}

fn main() {
    {
        let nums = vec![1, 1, 1];
        let ans = 3;
        assert_eq!(min_operations(nums), ans);
    }

    {
        let nums = vec![1, 5, 2, 4, 1];
        let ans = 14;
        assert_eq!(min_operations(nums), ans);
    }

    {
        let nums = vec![8];
        let ans = 0;
        assert_eq!(min_operations(nums), ans);
    }
}
