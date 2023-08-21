/*
 * @Date: 2023-08-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-21
 * @FilePath: /algorithm/rust/2337_can_change/can_change.rs
 */

struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (start, target) = (start.into_bytes(), target.into_bytes());
        let len = start.len();
        let len = if len == target.len() {
            len
        } else {
            return false;
        };

        let (mut i, mut j) = (0, 0);

        while i < len && j < len {
            match (start[i], target[j]) {
                (b'L', b'L') if i >= j => {
                    i += 1;
                    j += 1;
                }
                (b'R', b'R') if i <= j => {
                    i += 1;
                    j += 1;
                }
                (b'_', _) => {
                    i += 1;
                }
                (_, b'_') => {
                    j += 1;
                }
                _ => {
                    return false;
                }
            }
        }

        while i < len && start[i] == b'_' {
            i += 1;
        }
        while j < len && target[j] == b'_' {
            j += 1;
        }

        return (i == len) && (i == j);
    }
}

fn main() {
    let tests = vec![
        ("_L__R__R_", "L______RR", true),
        ("R_L_", "__LR", false),
        ("_R", "R_", false),
    ];

    for (start, target, ans) in tests {
        assert_eq!(
            Solution::can_change(start.to_string(), target.to_string()),
            ans
        );
    }
}
