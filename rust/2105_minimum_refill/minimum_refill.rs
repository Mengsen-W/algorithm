/*
 * @Date: 2024-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-09
 * @FilePath: /algorithm/rust/2105_minimum_refill/minimum_refill.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut res = 0;
        let n = plants.len();
        let mut posa = 0; // 两人位置
        let mut posb = n - 1;
        let mut vala = capacity_a; // 两人剩余水量
        let mut valb = capacity_b;
        // 模拟相遇前的浇水过程
        while posa < posb {
            if vala < plants[posa] {
                res += 1;
                vala = capacity_a - plants[posa];
            } else {
                vala -= plants[posa];
            }
            posa += 1;
            if valb < plants[posb] {
                res += 1;
                valb = capacity_b - plants[posb];
            } else {
                valb -= plants[posb];
            }
            posb -= 1;
        }
        // 模拟相遇后可能的浇水过程
        if posa == posb {
            if vala >= valb && vala < plants[posa] {
                res += 1;
            }
            if vala < valb && valb < plants[posb] {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![2, 2, 3, 3], 5, 5, 1),
        (vec![2, 2, 3, 3], 3, 4, 2),
        (vec![5], 10, 8, 0),
    ];

    for (plants, capacity_a, capacity_b, ans) in tests {
        assert_eq!(
            Solution::minimum_refill(plants, capacity_a, capacity_b),
            ans
        );
    }
}
