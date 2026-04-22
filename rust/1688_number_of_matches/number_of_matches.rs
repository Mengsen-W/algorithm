/*
 * @Date: 2022-01-25 00:27:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-25 00:50:45
 */

pub fn number_of_matches1(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 1 {
        if n % 2 == 0 {
            ans += n / 2;
            n /= 2;
        } else {
            ans += (n - 1) / 2;
            n = (n - 1) / 2 + 1;
        }
    }
    ans
}

pub fn number_of_matches2(n: i32) -> i32 {
    n - 1
}

fn main() {
    assert_eq!(number_of_matches1(7), 6);
    assert_eq!(number_of_matches1(14), 13);
    assert_eq!(number_of_matches2(7), 6);
    assert_eq!(number_of_matches2(14), 13);
}
