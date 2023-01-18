/*
 * @Date: 2022-05-20 21:43:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-20 22:13:49
 * @FilePath: /algorithm/462_min_moves2/min_moves2.rs
 */

pub fn min_moves2(nums: Vec<i32>) -> i32 {
    #[inline]
    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let x = nums[right];
        let mut i = left as i32 - 1;
        for j in left..right {
            if nums[j] <= x {
                i += 1;
                nums.swap(i as usize, j);
            }
        }
        nums.swap((i + 1) as usize, right);
        (i + 1) as usize
    }

    #[inline]
    fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, index: usize) -> i32 {
        let q = random_partition(nums, left, right);
        if q == index {
            return nums[q];
        } else {
            if q < index {
                return quick_select(nums, q + 1, right, index);
            } else {
                return quick_select(nums, left, q - 1, index);
            }
        }
    }

    #[inline]
    fn random_partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let i = (right + left) / 2;
        nums.swap(i, right);
        partition(nums, left, right)
    }

    let mut nums = nums;
    let n = nums.len();
    let x = quick_select(&mut nums, 0, n - 1, n / 2);
    (0..n).fold(0, |acc, i| acc + (nums[i] - x).abs())
}

fn main() {
    assert_eq!(min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(min_moves2(vec![1, 10, 2, 9]), 16);
}
