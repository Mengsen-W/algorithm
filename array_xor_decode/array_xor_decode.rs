/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 22:42:00
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:59:56
 */

fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let size = encoded.len();
    let mut res: Vec<i32> = vec![0; size + 1];
    res[0] = first;
    for i in 0..size {
        res[i + 1] = encoded[i] ^ res[i];
    }
    res
}

fn main() {
    let mut encoded: Vec<i32>;
    let mut first;

    encoded = vec![1, 2, 3];
    first = 1;
    assert_eq!([1, 0, 2, 1].to_vec(), decode(encoded, first));

    encoded = vec![6, 2, 7, 3];
    first = 4;
    assert_eq!([4, 2, 0, 7, 4].to_vec(), decode(encoded, first));
}
