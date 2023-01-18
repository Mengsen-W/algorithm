/*
 * @Date: 2022-10-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-08
 * @FilePath: /algorithm/870_advantage_count/advantage_count.rs
 */

pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let n = nums1.len();
    let mut res = vec![-1; n];
    let mut arr1: Vec<(usize, &i32)> = nums1.iter().enumerate().collect();
    let mut arr2: Vec<(usize, &i32)> = nums2.iter().enumerate().collect();
    arr1.sort_by(|a, b| b.1.cmp(a.1));
    arr2.sort_by(|a, b| b.1.cmp(a.1));
    let mut index = 0;
    for v in arr2.iter() {
        if index < n && v.1 < arr1[index].1 {
            res[v.0] = *arr1[index].1;
            index += 1;
        }
    }

    for v in res.iter_mut() {
        if *v == -1 {
            *v = *arr1[index].1;
            index += 1;
        }
    }
    res
}

fn main() {
    {
        let nums1 = vec![2, 7, 11, 15];
        let nums2 = vec![1, 10, 4, 11];
        let ans = vec![2, 11, 7, 15];
        assert_eq!(advantage_count(nums1, nums2), ans);
    }

    {
        let nums1 = vec![12, 24, 8, 32];
        let nums2 = vec![13, 25, 32, 11];
        let ans = vec![24, 32, 8, 12];
        assert_eq!(advantage_count(nums1, nums2), ans)
    }
}
