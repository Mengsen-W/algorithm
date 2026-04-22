struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        use std::collections::{HashSet, VecDeque};

        let chars = s.bytes().collect::<Vec<u8>>();
        let n = chars.len();
        let mut ans = s.clone();
        let mut q = VecDeque::<String>::new();
        q.push_back(ans.clone());
        let mut visited = HashSet::<String>::new();
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            if ans > cur {
                ans = cur.clone();
            }
            let mut ca = cur.bytes().collect::<Vec<u8>>();
            for i in (1..n).step_by(2) {
                ca[i] = (ca[i] - b'0' + a as u8) % 10 + b'0';
            }
            let added = String::from_utf8(ca).unwrap();
            if visited.insert(added.clone()) {
                q.push_back(added);
            }
            let mut rotated = String::new();
            rotated.push_str(&cur[n - b as usize..]);
            rotated.push_str(&cur[..n - b as usize]);
            if visited.insert(rotated.clone()) {
                q.push_back(rotated);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        ("5525", 9, 2, "2050"),
        ("74", 5, 1, "24"),
        ("0011", 4, 2, "0011"),
        ("43987654", 7, 3, "00553311"),
    ];

    for (s, a, b, expected) in tests {
        assert_eq!(
            Solution::find_lex_smallest_string(s.to_string(), a, b),
            expected
        );
    }
}
