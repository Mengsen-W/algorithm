struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1: i64 = 0;
        let mut sum2: i64 = 0;
        let mut zero1 = 0;
        let mut zero2 = 0;

        for &x in &nums1 {
            sum1 += x as i64;
            if x == 0 {
                sum1 += 1;
                zero1 += 1;
            }
        }

        for &x in &nums2 {
            sum2 += x as i64;
            if x == 0 {
                sum2 += 1;
                zero2 += 1;
            }
        }

        if (zero1 == 0 && sum2 > sum1) || (zero2 == 0 && sum1 > sum2) {
            return -1;
        }

        sum1.max(sum2)
    }
}

fn main() {
    let tests = vec![
        (vec![3, 2, 0, 1, 0], vec![6, 5, 0], 12),
        (vec![2, 0, 2, 0], vec![1, 4], -1),
    ];

    for (nums1, nums2, ans) in tests {
        assert_eq!(Solution::min_sum(nums1, nums2), ans);
    }
}
