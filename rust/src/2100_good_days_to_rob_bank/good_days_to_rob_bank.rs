/*
 * @Date: 2022-03-06 00:57:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-06 02:02:32
 * @FilePath: /algorithm/2100_good_days_to_rob_bank/good_days_to_rob_bank.rs
 */

pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let n = security.len();

    let (left, right) = (1..n).fold((vec![0; n], vec![0; n]), |(mut left, mut right), i| {
        if security[i] <= security[i - 1] {
            left[i] = left[i - 1] + 1;
        }
        if security[n - i - 1] <= security[n - i] {
            right[n - i - 1] = right[n - i] + 1;
        }
        (left, right)
    });

    (time..n as i32 - time).fold(Vec::new(), |mut ans, i| {
        if left[i as usize] >= time && right[i as usize] >= time {
            ans.push(i);
        }
        ans
    })
}

fn main() {
    assert_eq!(
        good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2),
        vec![2, 3]
    );

    assert_eq!(
        good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0),
        vec![0, 1, 2, 3, 4]
    );

    assert_eq!(good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2), vec![]);
    assert_eq!(good_days_to_rob_bank(vec![1], 5), vec![]);
}
