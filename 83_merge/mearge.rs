/*
 * @Date: 2021-04-05 10:27:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-05 10:35:57
 * @FilePath: \algorithm\83_merge\mearge.rs
 * @Description: file content
 */

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let mut right = nums1.len();
    while n > 0 {
        right -= 1;
        if m == 0 || nums1[m - 1] < nums2[n - 1] {
            nums1[right] = nums2[n - 1];
            if n > 0 {
                n -= 1
            }
        } else {
            nums1.swap(m - 1, right);
            if m > 0 {
                m -= 1
            }
        }
    }
}

fn main() {
    {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 2;
        merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }
    {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2: Vec<i32> = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }
}
