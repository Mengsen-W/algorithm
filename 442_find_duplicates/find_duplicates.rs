/*
 * @Date: 2022-05-08 08:04:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-08 08:17:10
 * @FilePath: /algorithm/442_find_duplicates/find_duplicates.rs
 */

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    (0..nums.len()).fold(Vec::new(), |mut ret, i| {
        let j = nums[i].abs() as usize - 1;
        nums[j] = -nums[j];
        if nums[j] > 0 {
            ret.push(j as i32 + 1);
        }
        ret
    })
}

fn main() {
    assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    assert_eq!(find_duplicates(vec![1]), vec![]);
}
