/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-02 15:56:20
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-02 16:07:08
 */

use std::collections::HashSet;

fn buddy_strings(a: String, b: String) -> bool {
    if a.len() != b.len() || a.len() < 2 {
        return false;
    }

    let mut index = [0, 0];
    let mut count = 0;
    let (va, vb) = (a.into_bytes(), b.into_bytes());
    let mut set = HashSet::new();
    let mut f = true;

    for (i, (a, b)) in va.iter().zip(vb.iter()).enumerate() {
        if count == 2 {
            return false;
        }

        if a != b {
            index[count] = i;
            count += 1;
        } else if a != &va[0] {
            f = false;
        }

        set.insert(a);
    }

    //println!("{:?} {:?} {:?}", f, count, va[index[0]] == vb[index[1]] && va[index[1]] == vb[index[0]]);
    // "abab"
    // "abab"
    match count {
        0 => f || set.len() < va.len(),
        1 => false,
        2 => va[index[0]] == vb[index[1]] && va[index[1]] == vb[index[0]],
        _ => unreachable!(),
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
}
