/*
 * @Date: 2022-05-04 08:11:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-04 08:22:06
 * @FilePath: /algorithm/1823_find_the_winner/find_the_winner.rs
 */

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    (2..=n).fold(1, |mut winner, i| {
        winner = (k + winner - 1) % i + 1;
        winner
    })
}

fn main() {
    assert_eq!(find_the_winner(5, 2), 3);
    assert_eq!(find_the_winner(6, 5), 1);
}
