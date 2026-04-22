/*
 * @Date: 2021-04-16 09:24:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-16 09:29:40
 */

fn is_scramble(s1: String, s2: String) -> bool {
    let len = if s1.len() == s2.len() {
        s1.len()
    } else {
        return false;
    };

    let mut f = vec![vec![vec![false; len]; len]; len + 1];
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());

    s1.iter().enumerate().for_each(|(i, &x)| {
        s2.iter().enumerate().for_each(|(j, &y)| {
            f[1][i][j] = x == y;
        });
    });

    (1..=len).for_each(|n| {
        (0..=(len - n)).for_each(|i| {
            (0..=(len - n)).for_each(|j| {
                for k in 1..n {
                    if (f[k][i][j] && f[n - k][i + k][j + k])
                        || (f[k][i][j + n - k] && f[n - k][i + k][j])
                    {
                        f[n][i][j] = true;
                        break;
                    }
                }
            });
        });
    });

    f[len][0][0]
}

fn main() {
    assert!(is_scramble(String::from("great"), "rgeat".to_string()));
    assert!(!is_scramble(String::from("abcde"), "caebd".to_string()));
    assert!(is_scramble(String::from("a"), "a".to_string()));
}
