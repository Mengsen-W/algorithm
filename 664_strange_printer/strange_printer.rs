/*
 * @Date: 2021-05-24 09:43:02
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 09:53:30
 */

fn strange_printer(s: String) -> i32 {
    let s = s.into_bytes();
    let n = s.len();
    let mut f = [[0; 100]; 100];
    for i in (0..n).rev() {
        f[i][i] = 1;
        for j in i + 1..n {
            f[i][j] = if s[i] == s[j] {
                f[i][j - 1]
            } else {
                (i..j).fold(std::i32::MAX, |s, k| s.min(f[i][k] + f[k + 1][j]))
            }
        }
    }
    return f[0][n - 1];
}

fn main() {
    assert_eq!(strange_printer("aaabbb".to_string()), 2);
    assert_eq!(strange_printer("aba".to_string()), 2);
}
