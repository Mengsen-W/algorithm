/*
 * @Date: 2021-07-12 08:24:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:26:22
 */

fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    let len = citations.len();
    for x in citations.iter().enumerate().map(|(i, value)| {
        if *value as usize >= len - i {
            return len - i;
        }
        return 99999999;
    }) {
        if x != 99999999 {
            return x as i32;
        }
    }
    return 0;
}

fn main() {
    let citations = vec![0, 1, 3, 5, 6];
    assert_eq!(h_index(citations), 3);

    let citations = vec![1];
    assert_eq!(h_index(citations), 1);
}
