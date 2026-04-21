/*
 * @Date: 2023-03-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-23
 * @FilePath: /algorithm/rust/1630_check_arithmetic_subarrays/check_arithmetic_subarrays.rs
 */

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    fn check_arithmetic_subarrays(mut nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        if nums.len() == 2 {
            return true;
        }
        nums.sort();
        for i in 1..nums.len() {
            if nums[1] - nums[0] != nums[i] - nums[i - 1] {
                return false;
            }
        }
        true
    }
    (0..l.len())
        .map(|i| check_arithmetic_subarrays(nums[l[i] as usize..r[i] as usize + 1].to_vec()))
        .collect::<Vec<_>>()
}

fn main() {
    {
        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0, 2];
        let r = vec![2, 3, 5];
        let ans = vec![true, false, true];
        assert_eq!(check_arithmetic_subarrays(nums, l, r), ans)
    }

    {
        let nums = vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10];
        let l = vec![0, 1, 6, 4, 8, 7];
        let r = vec![4, 4, 9, 7, 9, 10];
        let ans = vec![false, true, false, false, true, true];
        assert_eq!(check_arithmetic_subarrays(nums, l, r), ans)
    }
}
