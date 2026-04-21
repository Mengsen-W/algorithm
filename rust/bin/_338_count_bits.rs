/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-03 09:33:09
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-03 09:45:53
 */

fn count_bits(num: i32) -> Vec<i32> {
    (1..=num as usize).fold(vec![0], |mut v, i| {
        v.push(&v[i & (i - 1)] + 1);
        v
    })
}

fn main() {
    assert_eq!(count_bits(2), vec![0, 1, 1]);
    assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
