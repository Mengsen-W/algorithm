/*
 * @Date: 2021-05-25 09:34:25
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-25 20:41:39
 */

fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    const N: usize = 1024;
    let k = k as usize;
    let n = nums.len();
    let mut group_amount = vec![0; k as usize];
    let mut group_record: Vec<HashMap<i32, i32>> = vec![HashMap::new(); k];

    for i in 0..n {
        group_amount[i % k] += 1;
        let count = group_record[i % k].entry(nums[i]).or_insert(0);
        *count += 1;
    }

    let mut dp = vec![vec![0; N]; k];
    for j in 0..N {
        let count = group_record[0].entry(j as i32).or_insert(0);
        dp[0][j] = group_amount[0] - *count;
    }

    for i in 1..k {
        let upper_limit = dp[i - 1].iter().fold(i32::MAX, |a, b| a.min(*b)) + group_amount[i];
        dp[i].fill(upper_limit);
        // for _fill in dp[i].iter_mut() {
        //   *_fill = upper_limit;
        // }

        for (num, amount) in group_record[i % k].iter() {
            let num = *num as usize;
            for j in 0..N {
                dp[i][j ^ num] = dp[i][j ^ num].min(dp[i - 1][j] + group_amount[i] - amount);
            }
        }
    }
    dp[k - 1][0]
}

fn main() {
    assert_eq!(min_changes(vec![1, 2, 0, 3, 0], 1), 3);
    assert_eq!(min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
    assert_eq!(min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
}
