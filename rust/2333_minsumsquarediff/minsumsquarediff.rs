struct Solution;

impl Solution {
    pub fn min_sum_square_diff(mut nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let n = nums1.len();
        let mut k = k1 + k2;
        let mut max_dif = 0;
        for i in 0..n {
            nums1[i] = (nums1[i] - nums2[i]).abs();
            max_dif = max_dif.max(nums1[i]);
        }

        let (mut l, mut r, mut res) = (0, max_dif, 0);
        while l <= r {
            let mid = (l + r) >> 1;
            let sum: i64 = nums1.iter().map(|&num| (num.max(mid) - mid) as i64).sum();
            if sum <= k as i64 {
                r = mid - 1;
                res = mid;
            } else {
                l = mid + 1;
            }
        }

        for &num in &nums1 {
            if num > res {
                k -= num - res;
            }
        }

        nums1.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = 0i64;
        for &num in &nums1 {
            let mut diff = num.min(res) as i64;
            if k > 0 && diff > 0 {
                diff -= 1;
                k -= 1;
            }
            ans += diff * diff;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0, 579),
        (vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1, 43),
    ];

    for (nums1, nums2, k1, k2, expected) in tests {
        assert_eq!(
            Solution::min_sum_square_diff(nums1, nums2, k1, k2),
            expected
        );
    }
}
