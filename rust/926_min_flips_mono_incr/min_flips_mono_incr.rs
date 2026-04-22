/*
 * @Date: 2022-06-11 21:44:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-11 22:03:09
 * @FilePath: /algorithm/926_min_flips_mono_incr/min_flips_mono_incr.rs
 */

pub fn min_flips_mono_incr(s: String) -> i32 {
    let (mut dp0, mut dp1) = (0, 0);
    for c in s.chars() {
        let mut dp0_new = dp0;
        let mut dp1_new = dp0.min(dp1);
        if c == '1' {
            dp0_new += 1;
        } else {
            dp1_new += 1;
        }
        dp0 = dp0_new;
        dp1 = dp1_new;
    }
    dp0.min(dp1)
}

fn main() {
    assert_eq!(min_flips_mono_incr(String::from("00110")), 1);
    assert_eq!(min_flips_mono_incr(String::from("010110")), 2);
    assert_eq!(min_flips_mono_incr(String::from("00011000")), 2);
}
