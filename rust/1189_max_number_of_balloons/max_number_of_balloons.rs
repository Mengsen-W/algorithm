/*
 * @Date: 2022-02-13 01:07:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-13 01:32:44
 */

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut cnt = [0; 5];
    text.chars().for_each(|c| match c {
        'b' => cnt[0] += 1,
        'a' => cnt[1] += 1,
        'l' => cnt[2] += 1,
        'o' => cnt[3] += 1,
        'n' => cnt[4] += 1,
        _ => (),
    });
    cnt[2] >>= 1;
    cnt[3] >>= 1;
    *cnt.iter().min().unwrap_or(&0)
}

fn main() {
    assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
}
