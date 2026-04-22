/*
 * @Date: 2021-09-19 09:01:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-19 09:24:30
 */

struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut f: Vec<i32> = vec![0; n + 1];
        for i in 2..=n {
            f[i] = i32::MAX;
            let mut _j = 1;
            while _j * _j <= i {
                if i % _j == 0 {
                    f[i] = std::cmp::min(f[i], f[_j] + i as i32 / _j as i32);
                    f[i] = std::cmp::min(f[i], f[i / _j] + _j as i32);
                }
                _j += 1;
            }
        }
        f[n]
    }
}

fn main() {
    assert_eq!(Solution::min_steps(3), 3);
    assert_eq!(Solution::min_steps(1), 0);
}
