/*
 * @Date: 2023-05-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-29
 * @FilePath: /algorithm/rust/2455_average_value/average_value.rs
 */

pub fn average_value(nums: Vec<i32>) -> i32 {
    let (count, sum) = nums.into_iter().fold((0, 0), |(count, sum), num| {
        if num % 6 == 0 {
            (count + 1, sum + num)
        } else {
            (count, sum)
        }
    });
    if count == 0 {
        0
    } else {
        sum / count
    }
}

fn main() {
    {
        let nums = vec![1, 3, 6, 10, 12, 15];
        let ans = 9;
        assert_eq!(average_value(nums), ans);
    }

    {
        let nums = vec![1, 2, 4, 7, 10];
        let ans = 0;
        assert_eq!(average_value(nums), ans);
    }
}
