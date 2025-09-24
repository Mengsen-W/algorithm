struct Solution;

impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let n1 = nums1.len() as i32;
        let n2 = nums2.len() as i32;
        let mut pos1 = 0;
        let mut pos2 = 0;
        while pos1 < n1 && nums1[pos1 as usize] < 0 {
            pos1 += 1;
        }
        while pos2 < n2 && nums2[pos2 as usize] < 0 {
            pos2 += 1;
        }
        let mut left = -1e10 as i64;
        let mut right = 1e10 as i64;

        while left <= right {
            let mid = (left + right) / 2;
            let mut count = 0 as i64;
            let mut i1 = 0;
            let mut i2 = pos2 - 1;
            while i1 < pos1 && i2 >= 0 {
                if (nums1[i1 as usize] as i64) * (nums2[i2 as usize] as i64) > mid {
                    i1 += 1;
                } else {
                    count += (pos1 - i1) as i64;
                    i2 -= 1;
                }
            }
            let mut i1 = pos1;
            let mut i2 = n2 - 1;
            while i1 < n1 && i2 >= pos2 {
                if (nums1[i1 as usize] as i64) * (nums2[i2 as usize] as i64) > mid {
                    i2 -= 1;
                } else {
                    count += (i2 - pos2 + 1) as i64;
                    i1 += 1;
                }
            }
            let mut i1 = 0;
            let mut i2 = pos2;
            while i1 < pos1 && i2 < n2 {
                if (nums1[i1 as usize] as i64) * (nums2[i2 as usize] as i64) > mid {
                    i2 += 1;
                } else {
                    count += (n2 - i2) as i64;
                    i1 += 1;
                }
            }
            let mut i1 = pos1;
            let mut i2 = 0;
            while i1 < n1 && i2 < pos2 {
                if (nums1[i1 as usize] as i64) * (nums2[i2 as usize] as i64) > mid {
                    i1 += 1;
                } else {
                    count += (n1 - i1) as i64;
                    i2 += 1;
                }
            }
            if count < k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

fn main() {
    let tests = vec![
        (vec![2, 5], vec![3, 4], 2, 8),
        (vec![-4, -2, 0, 3], vec![2, 4], 6, 0),
        (vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3, -6),
    ];

    for (nums1, nums2, k, expected) in tests {
        assert_eq!(Solution::kth_smallest_product(nums1, nums2, k), expected,);
    }
}
