/*
 * @Date: 2022-01-14 01:23:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-14 02:33:55
 */

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
    let (m, n) = (nums1.len(), nums2.len());
    let count = |target: i32| -> i64 {
        let mut cnt: i64 = 0;
        let (mut start, mut end) = (0, n as i32 - 1);
        while start < m && end >= 0 {
            if nums1[start as usize] + nums2[end as usize] > target {
                end -= 1;
            } else {
                cnt += end as i64 + 1;
                start += 1;
            }
        }
        cnt
    };

    let mut left = nums1[0] + nums2[0];
    let mut right = nums1.last().unwrap() + nums2.last().unwrap();
    let mut pair_sum = right;
    while left <= right {
        let mid = left + ((right - left) >> 1);
        if count(mid) < k as i64 {
            left = mid + 1;
        } else {
            pair_sum = mid;
            right = mid - 1;
        }
    }

    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut pos = n as i32 - 1;
    for i in 0..m {
        while pos >= 0 && nums1[i] + nums2[pos as usize] >= pair_sum {
            pos -= 1;
        }
        let mut j = 0;
        while j <= pos && k > 0 {
            ans.push(vec![nums1[i], nums2[j as usize]]);
            k -= 1;
            j += 1;
        }
    }
    pos = n as i32 - 1;
    let mut i: i32 = 0;
    while i < m as i32 && k > 0 {
        while pos >= 0 && nums1[i as usize] + nums2[pos as usize] > pair_sum {
            pos -= 1;
        }
        let mut j = i;
        while k > 0 && j >= 0 && nums1[j as usize] + nums2[pos as usize] == pair_sum {
            ans.push(vec![nums1[j as usize], nums2[pos as usize]]);
            k -= 1;
            j -= 1;
        }
        i += 1;
    }
    ans
}

fn main() {
    assert_eq!(
        k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![vec![1, 2], vec![1, 4], vec![1, 6]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec![vec![1, 1], vec![1, 1]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec![vec![1, 3], vec![2, 3]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 2, 4, 5, 6], vec![3, 5, 7, 9], 3),
        vec![vec![1, 3], vec![2, 3], vec![1, 5]]
    );
}
