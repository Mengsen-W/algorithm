/*
 * @Date: 2022-03-02 00:10:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-02 01:25:39
 * @FilePath: /algorithm/564_nearest_palindromic/nearest_palindromic.rs
 */

pub fn nearest_palindromic(n: String) -> String {
    let m = n.len();
    let self_prefix = n[..(m + 1) / 2].parse::<i64>().unwrap();
    let self_number = n.parse::<i64>().unwrap();

    [self_prefix - 1, self_prefix, self_prefix + 1]
        .iter()
        .fold(
            vec![
                (10_i64.pow(m as u32 - 1) - 1) as i64,
                (10_i64.pow(m as u32) + 1) as i64,
            ],
            |mut candidates, &(mut x)| {
                let mut y = x;
                if m & 1 == 1 {
                    y /= 10;
                }
                while y > 0 {
                    x = x * 10 + y % 10;
                    y /= 10;
                }
                candidates.push(x);
                candidates
            },
        )
        .iter()
        .fold(-1, |mut ans, &candidate| {
            if candidate != self_number {
                if ans == -1
                    || (candidate - self_number).abs() < (ans - self_number).abs()
                    || ((candidate - self_number).abs() == (ans - self_number).abs()
                        && candidate < self_number)
                {
                    ans = candidate;
                }
            }
            ans
        })
        .to_string()
}

fn main() {
    assert_eq!(
        nearest_palindromic(String::from("123")),
        String::from("121")
    );
    assert_eq!(nearest_palindromic(String::from("1")), String::from("0"));
}
