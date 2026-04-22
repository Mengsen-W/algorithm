/*
 * @Date: 2022-09-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-12
 * @FilePath: /algorithm/1608_special_array/special_array.rs
 */

pub fn special_array(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let n = nums.len();
    for i in 1..=n {
        if nums[i - 1] >= i as i32 && (i == n || nums[i] < i as i32) {
            return i as i32;
        }
    }
    -1
}

fn main() {
    assert_eq!(special_array(vec![3, 5]), 2);
    assert_eq!(special_array(vec![0, 0]), -1);
    assert_eq!(special_array(vec![0, 4, 3, 0, 4]), 3);
    assert_eq!(special_array(vec![3, 6, 7, 7, 0]), -1);
}
