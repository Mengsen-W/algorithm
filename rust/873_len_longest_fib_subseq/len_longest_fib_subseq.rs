/*
 * @Date: 2022-07-09
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-09
 * @FilePath: /algorithm/873_len_longest_fib_subseq/len_longest_fib_subseq.rs
 */

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let idx: HashMap<i32, usize> = arr.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let mut dp = HashMap::new();
    let mut ans = 0;
    for k in 0..arr.len() {
        for j in 0..k {
            if arr[k] - arr[j] >= arr[j] {
                continue;
            }
            if let Some(&i) = idx.get(&(arr[k] - arr[j])) {
                let cnt = match dp.get(&(i, j)) {
                    Some(&n) => n,
                    _ => 0,
                };
                dp.insert((j, k), cnt + 1);
                ans = ans.max(dp[&(j, k)] + 2);
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    assert_eq!(len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
}
