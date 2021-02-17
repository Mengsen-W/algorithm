/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 10:33:05
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 10:34:05
 */

pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if nums.len() * nums[0].len() == (r * c) as usize {
        nums.concat() // 合并
            .chunks(c as usize) // 得到组迭代器
            .map(|v| v.to_vec()) // 变成 Vec
            .collect() // 组合
    } else {
        nums
    }
}

fn main() {
    let nums: Vec<Vec<i32>>;
    nums = vec![vec![1, 2], vec![3, 4]];
    println!("{:?}", matrix_reshape(nums.clone(), 1, 4));
    println!("{:?}", matrix_reshape(nums.clone(), 2, 4));
}
