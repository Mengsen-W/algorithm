/*
 * @Date: 2021-04-20 08:57:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-20 09:45:14
 */

fn kmp(haystack: String, needle: String) -> i32 {
    let haystack = haystack.into_bytes();
    let needle = needle.into_bytes();
    let _h = haystack.len();
    let _n = needle.len();

    if _n == 0 {
        return 0;
    }

    let mut next = vec![0];

    for i in 1.._n {
        let mut j = next[i - 1];
        while j > 0 && needle[i] != needle[j] {
            j = next[j - 1];
        }
        next.push(if needle[i] == needle[j] { j + 1 } else { j });
    }

    let mut j = 0;

    for (i, &c) in haystack.iter().enumerate() {
        while j > 0 && c != needle[j] {
            j = next[j - 1];
        }
        if needle[j] == c {
            j += 1;
        }
        if j == _n {
            return (i + 1 - j) as i32;
        }
    }

    -1
}

fn main() {
    assert_eq!(kmp("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(kmp("aaaaa".to_string(), "bba".to_string()), -1);
    assert_eq!(kmp("".to_string(), "".to_string()), 0);
}
