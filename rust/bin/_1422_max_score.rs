/*
 * @Date: 2022-08-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-14
 * @FilePath: /algorithm/1422_max_score/max_score.rs
 */

pub fn max_score(s: String) -> i32 {
    let (mut ret, mut ones_cnt, mut zeros_cnt) = (i32::MIN, 0, 0);
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b'1' {
            ones_cnt += 1
        } else {
            zeros_cnt += 1
        }
        if i != s.len() - 1 {
            ret = ret.max(zeros_cnt - ones_cnt)
        }
    }
    ones_cnt + ret
}

fn main() {
    assert_eq!(max_score(String::from("011101")), 5);
    assert_eq!(max_score(String::from("00111")), 5);
    assert_eq!(max_score(String::from("1111")), 3);
}
