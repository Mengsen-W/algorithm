/*
 * @Date: 2021-12-01 00:26:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-01 00:42:59
 */

pub fn max_power(s: String) -> i32 {
    let (mut ans, mut cnt, s_len): (usize, usize, usize) = (1, 1, s.len());
    for i in 1..s_len {
        if s.as_bytes()[i] == s.as_bytes()[i - 1] {
            cnt += 1;
            ans = std::cmp::max(ans, cnt);
        } else {
            cnt = 1;
        }
    }
    ans as i32
}

fn main() {
    assert_eq!(max_power("leetcode".to_string()), 2);
    assert_eq!(max_power("abbcccddddeeeeedcba".to_string()), 5);
    assert_eq!(max_power("triplepillooooow".to_string()), 5);
    assert_eq!(max_power("hooraaaaaaaaaaay".to_string()), 11);
}
