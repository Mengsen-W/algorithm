/*
 * @Date: 2022-08-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-04
 * @FilePath: /algorithm/1403_min_subsequence/min_subsequence.rs
 */

pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
    let (half_sum, mut pre) = (nums.iter().sum::<i32>() / 2, 0);
    nums.sort_by(|a, b| b.cmp(a));
    for i in 0..nums.len() {
        pre += nums[i];
        if pre > half_sum {
            return nums[..=i].to_vec();
        }
    }
    vec![]
}

fn main() {
    {
        let nums = vec![4, 3, 10, 9, 8];
        let ans = vec![10, 9];
        assert_eq!(min_subsequence(nums), ans);
    }

    {
        let nums = vec![4, 4, 7, 6, 7];
        let ans = vec![7, 7, 6];
        assert_eq!(min_subsequence(nums), ans);
    }
}
