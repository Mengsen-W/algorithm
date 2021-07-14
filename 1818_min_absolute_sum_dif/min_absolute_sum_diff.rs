/*
 * @Date: 2021-07-14 08:27:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-14 08:51:50
 */

fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut max_diff = 0;
    let mut ans = 0i64;
    let n = nums1.len();
    let mut ss = nums1.clone();
    ss.sort_unstable();
    for i in 0..n {
        let cur = (nums1[i] - nums2[i]).abs();
        ans += (nums1[i] - nums2[i]).abs() as i64;
        max_diff = max_diff.max({
            let c = ss.binary_search(&nums2[i]);
            let c = ss[match c {
                Ok(ci) => ci,
                Err(ci) => {
                    if ci == n {
                        ci - 1
                    } else if ci == 0 {
                        ci
                    } else if (ss[ci] - nums2[i]).abs() > (ss[ci - 1] - nums2[i]).abs() {
                        ci - 1
                    } else {
                        ci
                    }
                }
            }];
            cur - (c - nums2[i]).abs()
        });
    }
    ans -= max_diff as i64;
    (ans % 1_000_000_007) as i32
}

fn main() {
    {
        let nums1 = vec![1, 7, 5];
        let nums2 = vec![2, 3, 5];
        assert_eq!(min_absolute_sum_diff(nums1, nums2), 3);
    }
    {
        let nums1 = vec![2, 4, 6, 8, 10];
        let nums2 = vec![2, 4, 6, 8, 10];
        assert_eq!(min_absolute_sum_diff(nums1, nums2), 0);
    }
    {
        let nums1 = vec![1, 10, 4, 4, 2, 7];
        let nums2 = vec![9, 3, 5, 1, 7, 4];
        assert_eq!(min_absolute_sum_diff(nums1, nums2), 20);
    }
}
