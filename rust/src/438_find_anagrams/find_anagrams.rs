/*
 * @Date: 2021-11-28 02:17:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-28 02:28:13
 */

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut ans = vec![];
    let mut tab = [0; 26];
    let mut cur = [0; 26];
    let (n, m) = (s.len(), p.len());
    for c in p.into_bytes() {
        tab[c as usize - b'a' as usize] += 1;
    }
    let s = s.into_bytes();
    for i in 0..(m - 1).min(n) {
        cur[s[i] as usize - b'a' as usize] += 1;
    }
    for i in m - 1..n {
        cur[s[i] as usize - b'a' as usize] += 1;
        let mut ok = true;
        for (x, y) in cur.iter().zip(tab.iter()) {
            if x != y {
                ok = false;
                break;
            }
        }
        if ok {
            ans.push((i + 1 - m) as i32)
        }
        cur[s[i + 1 - m] as usize - b'a' as usize] -= 1;
    }
    ans
}

fn main() {
    assert_eq!(
        find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}
