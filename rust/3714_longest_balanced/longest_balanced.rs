struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let n = s.len();
        let mut res = 0;
        let bytes = s.as_bytes();

        // 情况一：仅包含一种字符
        let mut last = 0;
        for i in 0..n {
            last = if i > 0 && bytes[i] == bytes[i - 1] {
                last + 1
            } else {
                1
            };
            res = res.max(last as i32);
        }

        // 情况二：包含两种字符
        let helper = |x: u8, y: u8| -> i32 {
            let mut ans = 0;
            let mut i = 0;
            while i < n {
                if bytes[i] != x && bytes[i] != y {
                    i += 1;
                    continue;
                }

                let mut h = HashMap::new();
                h.insert(0, i as i32 - 1);
                let mut diff = 0;

                while i < n && (bytes[i] == x || bytes[i] == y) {
                    diff += if bytes[i] == x { 1 } else { -1 };
                    match h.get(&diff) {
                        Some(&pos) => ans = ans.max(i as i32 - pos),
                        None => {
                            h.insert(diff, i as i32);
                        }
                    }
                    i += 1;
                }
                i += 1;
            }
            ans
        };

        res = res.max(helper(b'a', b'b'));
        res = res.max(helper(b'b', b'c'));
        res = res.max(helper(b'a', b'c'));

        // 情况三：包含三种字符

        let mut pre = [0; 3];
        let mut h = HashMap::new();
        h.insert((0, 0), -1);

        for (i, &ch) in bytes.iter().enumerate() {
            pre[(ch - b'a') as usize] += 1;
            let key = (pre[1] - pre[0], pre[1] - pre[2]);

            match h.get(&key) {
                Some(&pos) => res = res.max(i as i32 - pos),
                None => {
                    h.insert(key, i as i32);
                }
            }
        }

        res
    }
}

fn main() {
    let tests = vec![("abbac", 4), ("aabcc", 3), ("aba", 2)];

    for (s, ans) in tests {
        assert_eq!(Solution::longest_balanced(s.to_string()), ans);
    }
}
