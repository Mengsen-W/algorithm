/*
 * @Date: 2021-12-27 01:43:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-27 02:16:56
 */

struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut cnt: Vec<i32> = vec![0; 121];
        let mut pre_sum: Vec<i32> = vec![0; 121];
        let mut ans = 0;

        ages.iter().for_each(|&x| cnt[x as usize] += 1);
        (1..121).for_each(|i| pre_sum[i] = pre_sum[i - 1] + cnt[i]);
        (15..121).for_each(|i| {
            ans += (pre_sum[i - 1] - pre_sum[i / 2 + 7]) * cnt[i] + cnt[i] * (cnt[i] - 1)
        });
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![16, 16], 2),
        (vec![16, 17, 18], 2),
        (vec![20, 30, 100, 110, 120], 3),
    ];

    for (ages, ans) in tests {
        assert_eq!(Solution::num_friend_requests(ages), ans);
    }
}
