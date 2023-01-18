/*
 * @Date: 2022-11-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-27
 * @FilePath: /algorithm/1752_check/check.rs
 */

pub fn check(nums: Vec<i32>) -> bool {
    let mut a = 2;
    for i in nums.windows(2) {
        if i[0] > i[1] {
            a -= 1;
            if nums[0] < nums[nums.len() - 1] {
                return false;
            }
            if a == 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    {
        let nums = vec![3, 4, 5, 1, 2];
        assert!(check(nums));
    }

    {
        let nums = vec![2, 1, 3, 4];
        assert!(!check(nums))
    }

    {
        let nums = vec![1, 2, 3];
        assert!(check(nums));
    }
}
