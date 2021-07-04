/*
 * @Date: 2021-07-04 10:23:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-04 15:44:23
 */

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let (n, mut xor_sum) = (nums.len(), 0);

    nums.iter().for_each(|num| xor_sum ^= num);
    (1..=n as i32).for_each(|i| xor_sum ^= i);

    let (low_bit, mut num1, mut num2) = (xor_sum & (-xor_sum), 0, 0);

    nums.iter().for_each(|num| match num & low_bit {
        0 => num1 ^= num,
        _ => num2 ^= num,
    });

    (1..=n as i32).for_each(|i| match i & low_bit {
        0 => num1 ^= i,
        _ => num2 ^= i,
    });

    for num in nums.iter() {
        if *num == num1 {
            return vec![num1, num2];
        }
    }
    vec![num2, num1]
}

fn main() {
    {
        let nums = vec![1, 2, 2, 4];
        assert_eq!(find_error_nums(nums), vec![2, 3]);
    }
    {
        let nums = vec![1, 1];
        assert_eq!(find_error_nums(nums), vec![1, 2]);
    }
}
