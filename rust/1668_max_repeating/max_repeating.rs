/*
 * @Date: 2022-11-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-03
 * @FilePath: /algorithm/1668_max_repeating/max_repeating.rs
 */

pub fn max_repeating(sequence: String, word: String) -> i32 {
    let (sequence, word): (Vec<char>, Vec<char>) =
        (sequence.chars().collect(), word.chars().collect());
    let (n, m) = (sequence.len(), word.len());
    if n < m {
        return 0;
    }

    let mut fail: Vec<i32> = vec![-1; m];
    for i in 1..m {
        let mut j = fail[i - 1];
        while j != -1 && word[(j + 1) as usize] != word[i] {
            j = fail[j as usize];
        }
        if word[(j + 1) as usize] == word[i] {
            fail[i] = j + 1;
        }
    }

    let mut f: Vec<i32> = vec![0; n];
    let mut j: i32 = -1;
    for i in 0..n {
        while j != -1 && word[(j + 1) as usize] != sequence[i] {
            j = fail[j as usize];
        }
        if word[(j + 1) as usize] == sequence[i] {
            j += 1;
            if j as usize == m - 1 {
                f[i] = if i >= m { f[i - m] } else { 0 } + 1;
                j = fail[j as usize];
            }
        }
    }

    *f.iter().max().unwrap()
}

fn main() {
    assert_eq!(max_repeating(String::from("ababc"), String::from("ab")), 2);
    assert_eq!(max_repeating(String::from("ababc"), String::from("bc")), 1);
    assert_eq!(max_repeating(String::from("ababc"), String::from("ac")), 0);
}
