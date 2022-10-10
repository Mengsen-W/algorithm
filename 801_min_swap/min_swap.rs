/*
 * @Date: 2022-10-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-10
 * @FilePath: /algorithm/801_min_swap/min_swap.rs
 */

pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len() as i32;
    let mut a = 0;
    let mut b = 1;
    for i in 1..n as usize {
        let ai = a;
        let bi = b;
        a = n;
        b = n;
        if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
            if ai < a {
                a = ai;
            }
            if bi + 1 < b {
                b = bi + 1;
            }
        }
        if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
            if bi < a {
                a = bi;
            }
            if ai + 1 < b {
                b = ai + 1;
            }
        }
    }
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    {
        let nums1 = vec![1, 3, 5, 4];
        let nums2 = vec![1, 2, 3, 7];
        let ans = 1;
        assert_eq!(min_swap(nums1, nums2), ans);
    }

    {
        let nums1 = vec![0, 3, 5, 8, 9];
        let nums2 = vec![2, 1, 4, 6, 9];
        let ans = 1;
        assert_eq!(min_swap(nums1, nums2), ans);
    }
}
