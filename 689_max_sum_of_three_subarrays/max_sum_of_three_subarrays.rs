/*
 * @Date: 2021-12-09 04:47:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-09 05:29:15
 */

pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    let (
        mut sum1,
        mut max_sum1,
        mut max_sum1_idx,
        mut sum2,
        mut max_sum12,
        mut max_sum12_idx1,
        mut max_sum12_idx2,
        mut sum3,
        mut max_total,
    ) = (0, 0, 0, 0, 0, 0, 0, 0, 0);
    for i in k * 2..nums.len() as i32 {
        sum1 += nums[(i - k * 2) as usize];
        sum2 += nums[(i - k) as usize];
        sum3 += nums[i as usize];

        if i >= k * 3 - 1 {
            if sum1 > max_sum1 {
                max_sum1 = sum1;
                max_sum1_idx = i - k * 3 + 1;
            }
            if max_sum1 + sum2 > max_sum12 {
                max_sum12 = max_sum1 + sum2;
                max_sum12_idx1 = max_sum1_idx;
                max_sum12_idx2 = i - k * 2 + 1;
            }

            if max_sum12 + sum3 > max_total {
                max_total = max_sum12 + sum3;
                ans = vec![max_sum12_idx1, max_sum12_idx2, (i - k + 1)];
            }

            sum1 -= nums[(i - k * 3 + 1) as usize];
            sum2 -= nums[(i - k * 2 + 1) as usize];
            sum3 -= nums[(i - k + 1) as usize];
        }
    }
    ans
}

fn main() {
    assert_eq!(
        max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
        vec![0, 3, 5]
    );
    assert_eq!(
        max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
        vec![0, 2, 4]
    );
}
