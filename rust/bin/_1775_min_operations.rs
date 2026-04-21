/*
 * @Date: 2022-12-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-07
 * @FilePath: /algorithm/1775_min_operations/min_operations.rs
 */

pub fn min_operations(a: Vec<i32>, b: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    use std::mem::swap;
    if a.len() * 6 < b.len() || b.len() * 6 < a.len() {
        return -1;
    }
    let (mut a, mut b) = (a, b);
    let (mut s1, mut s2): (i32, i32) = (a.iter().sum(), b.iter().sum());
    let mut pq = BinaryHeap::new();
    if s1 < s2 {
        swap(&mut s1, &mut s2);
        swap(&mut a, &mut b);
    }
    a.iter().for_each(|&v| pq.push(v - 1));
    b.iter().for_each(|&v| pq.push(6 - v));
    let (mut ans, mut k) = (0, s1 - s2);
    while k > 0 {
        ans += 1;
        k -= pq.pop().unwrap();
    }
    ans
}

fn main() {
    {
        let nums1 = vec![1, 2, 3, 4, 5, 6];
        let nums2 = vec![1, 1, 2, 2, 2, 2];
        let ans = 3;
        assert_eq!(min_operations(nums1, nums2), ans);
    }

    {
        let nums1 = vec![1, 1, 1, 1, 1, 1, 1];
        let nums2 = vec![6];
        let ans = -1;
        assert_eq!(min_operations(nums1, nums2), ans);
    }

    {
        let nums1 = vec![6, 6];
        let nums2 = vec![1];
        let ans = 3;
        assert_eq!(min_operations(nums1, nums2), ans);
    }
}
