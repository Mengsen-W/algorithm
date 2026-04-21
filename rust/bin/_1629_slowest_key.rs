/*
 * @Date: 2022-01-09 01:04:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-09 01:39:39
 */

pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    std::iter::once(release_times[0])
        .chain(release_times.windows(2).map(|x| x[1] - x[0]))
        .zip(keys_pressed.chars())
        .max()
        .unwrap()
        .1
}

fn main() {
    assert_eq!(slowest_key(vec![9, 29, 49], "cbcd".to_string()), 'c');
    assert_eq!(
        slowest_key(vec![12, 23, 36, 46, 62], "supda".to_string()),
        'a'
    );
}
