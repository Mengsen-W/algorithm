/*
 * @Date: 2021-06-13 09:35:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-13 10:15:18
 */

fn is_bad_version(n: i32) -> bool {
    n == 4
}

fn first_bad_version(n: i32) -> i32 {
    let mut _left = 1;
    let mut _right = n;
    while _left < _right {
        let mid = _left + (_right - _left) / 2;
        if is_bad_version(mid) {
            _right = mid;
        } else {
            _left = mid + 1;
        }
    }
    _left
}

fn main() {
    assert_eq!(first_bad_version(5), 4);
}
