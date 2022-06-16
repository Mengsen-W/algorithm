/*
 * @Date: 2022-06-16 09:46:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 09:53:04
 * @FilePath: /algorithm/719_smallest_distance_pair/smallest_distance_pair.rs
 */

pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let n = nums.len();
    let mut lo = 0;
    let mut hi = nums[n - 1] - nums[0];

    fn bin_search(nums: &Vec<i32>, target: i32, bound: i32) -> i32 {
        let mut lo = 0;
        let mut hi = bound - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mut cnt = 0;

        for i in 0..n {
            let j = bin_search(&nums, nums[i] - mid, i as i32);
            cnt += i as i32 - j;
        }

        if cnt >= k {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    lo
}

fn main() {
    assert_eq!(smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(smallest_distance_pair(vec![1, 1, 1], 2), 0);
    assert_eq!(smallest_distance_pair(vec![1, 6, 1], 3), 5);
}
