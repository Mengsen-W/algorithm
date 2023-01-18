/*
 * @Date: 2021-05-28 09:53:00
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-28 10:27:56
 */

fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut zero;
    for i in 0..32 {
        zero = 0;
        for j in 0..nums.len() {
            if (nums[j] >> i) & 1 == 1 {
                zero += 1;
            }
        }
        result += (nums.len() - zero) * zero;
    }
    result as i32
}

fn main() {
    assert_eq!(total_hamming_distance(vec![4, 14, 2]), 6);
}
