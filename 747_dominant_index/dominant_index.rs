/*
 * @Date: 2022-01-13 01:33:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-13 02:31:52
 */

pub fn dominant_index(nums: Vec<i32>) -> i32 {
    if let Some((index, &max)) = nums.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)) {
        if nums
            .iter()
            .enumerate()
            .all(|(idx, num)| index == idx || num * 2 <= max)
        {
            return index as i32;
        }
    }

    -1
}

fn main() {
    assert_eq!(dominant_index(vec![3, 6, 1, 0]), 1);
    assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
    assert_eq!(dominant_index(vec![0, 0, 0, 1]), 3);
}
