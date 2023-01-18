/*
 * @Date: 2022-12-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-20
 * @FilePath: /algorithm/1760_minimum_size/minimum_size.rs
 */

pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let check = |ans: i32| -> bool {
        if ans == 0 {
            return false;
        }
        let mut times = 0;
        for i in &nums {
            if i % ans == 0 {
                times += i / ans - 1;
            } else {
                times += i / ans;
            }

            if times > max_operations {
                return false;
            }
        }
        times <= max_operations
    };

    let mut left = 0;
    let mut right = *nums.iter().max().unwrap();
    while left < right {
        let mid = left + ((right - left) >> 1);
        if check(mid as i32) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}

fn main() {
    {
        let nums = vec![9];
        let max_operations = 2;
        let ans = 3;
        assert_eq!(minimum_size(nums, max_operations), ans);
    }

    {
        let nums = vec![2, 4, 8, 2];
        let max_operations = 4;
        let ans = 2;
        assert_eq!(minimum_size(nums, max_operations), ans);
    }

    {
        let nums = vec![7, 17];
        let max_operations = 2;
        let ans = 7;
        assert_eq!(minimum_size(nums, max_operations), ans);
    }
}
