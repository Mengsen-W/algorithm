/*
 * @Date: 2021-07-09 09:12:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-09 11:37:40
 */

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut ret: (i32, i32) = (0, 0);
    let size = nums.len();
    (0..size).for_each(|i| {
        if ret.0 == nums[i] {
            ret.1 += 1;
        } else if ret.1 == 0 {
            ret.0 = nums[i];
            ret.1 = 1;
        } else {
            ret.1 -= 1;
        }
    });
    if nums.iter().filter(|&&num| num == ret.0).count() > size / 2 {
        return ret.0;
    } else {
        return -1;
    }
}

fn main() {
    {
        let nums = vec![1, 2, 5, 9, 5, 9, 5, 5, 5];
        assert_eq!(majority_element(nums), 5);
    }
    {
        let nums = vec![3, 2];
        assert_eq!(majority_element(nums), -1);
    }
    {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }
}
