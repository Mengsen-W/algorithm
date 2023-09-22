/*
 * @Date: 2023-09-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-22
 * @FilePath: /algorithm/rust/2591_dist_money/dist_money.rs
 */

struct Solution;
impl Solution {
    pub fn dist_money(sum: i32, child: i32) -> i32 {
        if sum < child {
            return -1;
        } else if sum == child {
            return 0;
        }
        let sum = sum - child;
        let div = sum / 7;
        let left = sum % 7;
        if div > child {
            child - 1
        } else if div == child {
            if left == 0 {
                return child;
            } else {
                return child - 1;
            }
        } else {
            if left == 3 {
                if div + 1 == child {
                    return child - 2;
                } else {
                    div
                }
            } else {
                div
            }
        }
    }
}

fn main() {
    let tests = vec![(20, 3, 1), (16, 2, 2)];

    for (sum, child, ans) in tests {
        assert_eq!(Solution::dist_money(sum, child), ans);
    }
}
