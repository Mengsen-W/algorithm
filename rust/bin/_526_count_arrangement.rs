/*
 * @Date: 2021-08-16 21:17:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-16 21:28:22
 */

struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut f = vec![0; 1 << n];
        f[0] = 1;
        let mut mask: i32 = 1;
        while mask < (1 << n) {
            let num = mask.count_ones();
            for i in 0..n {
                let num = num as i32;
                if (mask & (1 << i) != 0) && (num % (i + 1) == 0 || (i + 1) % num == 0) {
                    let mask = mask as usize;
                    f[mask] += f[mask ^ (1 << i)];
                }
            }
            mask += 1;
        }
        f[(1 << n) - 1]
    }
}

fn main() {
    assert_eq!(Solution::count_arrangement(2), 2);
}
