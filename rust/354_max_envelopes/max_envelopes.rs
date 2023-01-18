/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-04 09:54:28
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-04 09:55:57
 */

fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    if envelopes.len() == 0 {
        return 0;
    }
    let mut envelopes = envelopes;
    envelopes.sort();
    let mut dp = vec![1; envelopes.len()];
    for i in 1..envelopes.len() {
        for j in 0..i {
            if envelopes[i][0] > envelopes[j][0] && envelopes[i][1] > envelopes[j][1] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    assert_eq!(
        max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    )
}
