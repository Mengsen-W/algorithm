/*
 * @Date: 2022-01-12 00:48:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-12 01:08:06
 */

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 3 {
        return false;
    }
    let (mut first, mut second) = (nums[0], i32::MAX);
    for i in 1..n {
        let num = nums[i];
        if num > second {
            return true;
        } else if num > first {
            second = num;
        } else {
            first = num;
        }
    }
    false
}

fn main() {
    assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    assert_eq!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
}
