/*
 * @Date: 2022-06-13 09:52:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-13 10:06:59
 * @FilePath: /algorithm/1051_height_checker/height_checker.rs
 */

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let m = (*heights.iter().max().unwrap()) as usize;
    let cnt = heights.iter().fold(vec![0; m + 1], |mut acc, &h| {
        acc[h as usize] += 1;
        acc
    });

    let (mut idx, mut ans) = (0, 0);
    for i in 1..=m {
        for _ in 1..=cnt[i] {
            if heights[idx] != i as i32 {
                ans += 1;
            }
            idx += 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
}
