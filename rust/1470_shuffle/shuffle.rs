/*
 * @Date: 2022-08-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-29
 * @FilePath: /algorithm/1470_shuffle/shuffle.rs
 */

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    nums[0..n as usize]
        .into_iter()
        .zip(nums[n as usize..2 * n as usize].into_iter())
        .map(|(&a, &b)| vec![a, b])
        .flatten()
        .collect::<Vec<_>>()
}

fn main() {
    {
        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        let ans = vec![2, 3, 5, 4, 1, 7];
        assert_eq!(shuffle(nums, n), ans);
    }
    {
        let nums = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let n = 4;
        let ans = vec![1, 4, 2, 3, 3, 2, 4, 1];
        assert_eq!(shuffle(nums, n), ans);
    }
    {
        let nums = vec![1, 1, 2, 2];
        let n = 2;
        let ans = vec![1, 2, 1, 2];
        assert_eq!(shuffle(nums, n), ans);
    }
}
