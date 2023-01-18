/*
 * @Date: 2022-10-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-27
 * @FilePath: /algorithm/1822_array_sign/array_sign.rs
 */

pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut sign = 1;
    for num in nums {
        if num == 0 {
            return 0;
        }
        if num < 0 {
            sign = -sign;
        }
    }
    return sign;
}

fn main() {
    {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        let ans = 1;
        assert_eq!(array_sign(nums), ans);
    }

    {
        let nums = vec![1, 5, 0, 2, -3];
        let ans = 0;
        assert_eq!(array_sign(nums), ans);
    }

    {
        let nums = vec![-1, 1, -1, 1, -1];
        let ans = -1;
        assert_eq!(array_sign(nums), ans);
    }
}
