/*
 * @Date: 2022-02-21 00:58:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-21 01:14:20
 */

pub fn push_dominoes(dominoes: String) -> String {
    let mut dominoes = dominoes.chars().collect::<Vec<char>>();
    let (n, mut i) = (dominoes.len(), 0);
    let mut left = 'L';
    while i < n {
        let mut j = i;
        while j < n && dominoes[j] == '.' {
            j += 1;
        }
        let right = if j < n { dominoes[j] } else { 'R' };
        if left == right {
            while i < j {
                dominoes[i] = left;
                i += 1;
            }
        } else if left == 'R' && right == 'L' {
            let mut k = j - 1;
            while i < k {
                dominoes[i] = 'R';
                i += 1;
                dominoes[k] = 'L';
                k -= 1;
            }
        }
        left = right;
        i = j + 1;
    }
    dominoes.iter().collect::<String>()
}

fn main() {
    assert_eq!(push_dominoes(String::from("RR.L")), "RR.L".to_string());
    assert_eq!(
        push_dominoes(String::from(".L.R...LR..L..")),
        "LL.RR.LLRRLL..".to_string()
    );
}
