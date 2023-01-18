/*
 * @Date: 2022-02-10 00:35:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-10 00:55:12
 */

pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut ans = vec![];
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }
    for a in 2..=n {
        for b in 1..a {
            if gcd(b, a) == 1 {
                ans.push(format!("{}/{}", b, a));
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(simplified_fractions(2), vec!["1/2"]);
    assert_eq!(simplified_fractions(3), vec!["1/2", "1/3", "2/3"]);
    assert_eq!(
        simplified_fractions(4),
        vec!["1/2", "1/3", "2/3", "1/4", "3/4"]
    );
    assert_eq!(simplified_fractions(1), vec![""; 0]);
}
