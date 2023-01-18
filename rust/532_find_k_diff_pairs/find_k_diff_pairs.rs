/*
 * @Date: 2022-06-16 09:53:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 09:55:19
 * @FilePath: /algorithm/532_find_k_diff_pairs/find_k_diff_pairs.rs
 */

pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let (mut ans, mut y) = (0, 0);
    let n = nums.len();

    for x in 0..n {
        if x == 0 || nums[x] != nums[x - 1] {
            while y < n && (nums[y] < nums[x] + k || y <= x) {
                y += 1;
            }
            if y < n && nums[y] == nums[x] + k {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    assert_eq!(find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}
