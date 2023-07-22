/*
 * @Date: 2023-07-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-22
 * @FilePath: /algorithm/rust/860_lemonade_change/lemonade_change.rs
 */

struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);
        for v in bills.iter() {
            match v {
                5 => {
                    five += 1;
                }
                10 => {
                    five -= 1;
                    ten += 1;
                }
                _ => {
                    if ten != 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        five -= 3;
                    }
                }
            }

            if five < 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![5, 5, 5, 10, 20], true),
        (vec![5, 5, 10, 10, 20], false),
    ];

    for (bills, ans) in tests {
        assert_eq!(Solution::lemonade_change(bills), ans);
    }
}
