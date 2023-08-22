/*
 * @Date: 2023-08-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-22
 * @FilePath: /algorithm/rust/849_max_dist_to_closest/max_dist_to_closest.rs
 */

struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let (mut res, mut l) = (0, 0);
        while l < seats.len() && seats[l] == 0 {
            l += 1;
        }
        res = res.max(l);

        while l < seats.len() {
            let mut r = l + 1;
            while r < seats.len() && seats[r] == 0 {
                r += 1;
            }
            res = if r == seats.len() {
                res.max(r - l - 1)
            } else {
                res.max((r - l) / 2)
            };
            l = r;
        }
        return res as i32;
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 0, 0, 1, 0, 1], 2),
        (vec![1, 0, 0, 0], 3),
        (vec![0, 1], 1),
    ];

    for (seats, ans) in tests {
        assert_eq!(Solution::max_dist_to_closest(seats), ans);
    }
}
