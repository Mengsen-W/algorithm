/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-02 15:56:20
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-02 16:07:08
 */

pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    if s == goal {
        let mut mp = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            mp[idx] += 1;
            if mp[idx] > 1 {
                return true;
            }
        }
        false
    } else {
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        let (mut a, mut b) = (usize::MAX, usize::MAX);
        for i in 0..s.len() {
            if s[i] != goal[i] {
                if a == usize::MAX {
                    a = i;
                } else if b == usize::MAX {
                    b = i;
                } else {
                    return false;
                }
            }
        }
        a != usize::MAX && b != usize::MAX && s[a] == goal[b] && s[b] == goal[a]
    }
}

fn main() {
    assert!(buddy_strings("ab".to_string(), "ba".to_string()));
    assert!(!buddy_strings("ab".to_string(), "ab".to_string()));
    assert!(buddy_strings("aa".to_string(), "aa".to_string()));
    assert!(buddy_strings(
        "aaaaaaabc".to_string(),
        "aaaaaaacb".to_string()
    ));
    assert!(buddy_strings("abcd".to_string(), "bacd".to_string()));
}
