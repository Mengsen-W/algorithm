/*
 * @Date: 2021-04-30 09:55:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-30 10:25:30
 */

fn single_number(nums: Vec<i32>) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    nums.iter().for_each(|num| {
        b = !a & (b ^ num);
        a = !b & (a ^ num);
    });
    b
}

fn main() {
    {
        let nums = vec![2, 2, 3, 2];
        assert_eq!(single_number(nums), 3);
    }
    {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        assert_eq!(single_number(nums), 99);
    }
}
