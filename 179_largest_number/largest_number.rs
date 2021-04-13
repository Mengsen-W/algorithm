/*
 * @Date: 2021-04-12 08:55:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-12 09:21:09
 */

fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_by(|a, b| {
        let s1 = a.to_string() + &b.to_string();
        let s2 = b.to_string() + &a.to_string();

        s2.cmp(&s1)
    });

    if nums[0] == 0 {
        return "0".to_string();
    }

    nums.iter().map(|x| x.to_string()).collect::<String>()
}

fn main() {
    {
        let nums = vec![10, 2];
        assert_eq!(largest_number(nums), "210");
    }
    {
        let nums = vec![3, 30, 34, 5, 9];
        assert_eq!(largest_number(nums), "9534330");
    }
    {
        let nums = vec![1];
        assert_eq!(largest_number(nums), "1");
    }
    {
        let nums = vec![10];
        assert_eq!(largest_number(nums), "10");
    }
}
