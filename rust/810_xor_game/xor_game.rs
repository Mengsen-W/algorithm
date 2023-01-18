/*
 * @Date: 2021-05-24 10:20:05
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 10:25:29
 */

pub fn xor_game(nums: Vec<i32>) -> bool {
    if nums.len() % 2 == 0 {
        return true;
    }
    let mut s = 0;
    for num in nums.iter() {
        s ^= num;
    }
    s == 0
}

fn main() {
    assert!(!xor_game(vec![1, 1, 2]));
}
