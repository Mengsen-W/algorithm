/*
 * @Date: 2021-03-08 09:36:24
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-08 09:46:59
 */

fn min_cut(s: String) -> i32 {
    if s.len() < 1 {
        return 0;
    }

    let s: Vec<char> = s.chars().collect();
    let (mut f, mut p) = (Vec::new(), vec![vec![false; s.len()]; s.len()]);
    (0..=(s.len() as i32)).rev().for_each(|x| {
        f.push(x - 1);
    });

    (0..s.len()).rev().for_each(|i| {
        (i..s.len()).for_each(|j| {
            if s[i] == s[j] && (j - i < 2 || p[i + 1][j - 1]) {
                p[i][j] = true;
                f[i] = std::cmp::min(f[i], f[j + 1] + 1);
            }
        });
    });

    f[0]
}

fn main() {
    assert_eq!(min_cut("aab".to_string()), 1);
    assert_eq!(min_cut("a".to_string()), 0);
    assert_eq!(min_cut("ab".to_string()), 1);
}
