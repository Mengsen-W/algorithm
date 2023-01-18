/*
 * @Date: 2022-03-09 00:34:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-09 01:12:09
 * @FilePath: /algorithm/798_best_rotation/best_rotation.rs
 */

pub fn best_rotation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut diffs = vec![0; n];
    for i in 0..n {
        let low = (i + 1) % n;
        let high = (i + n + 1 - nums[i] as usize) % n;
        diffs[low] += 1;
        diffs[high] -= 1;
        if low >= high {
            diffs[0] += 1;
        }
    }
    let mut best_index = 0;
    let mut max_score = 0;
    let mut score = 0;
    for i in 0..n {
        score += diffs[i];
        if score > max_score {
            best_index = i;
            max_score = score;
        }
    }
    best_index as i32
}

fn main() {
    assert_eq!(best_rotation(vec![2, 3, 1, 4, 0]), 3);
    assert_eq!(best_rotation(vec![1, 3, 0, 2, 4]), 0);
}
