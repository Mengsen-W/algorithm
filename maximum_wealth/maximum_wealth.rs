/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 21:53:28
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:15:20
 */

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .into_iter()
        .map(|v| v.into_iter().sum())
        .max()
        .unwrap()
}

fn main() {
    let mut accounts: Vec<Vec<i32>>;
    accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    assert_eq!(6, maximum_wealth(accounts));
    accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    assert_eq!(10, maximum_wealth(accounts));
    accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
    assert_eq!(17, maximum_wealth(accounts));
}
