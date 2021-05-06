/*
 * @Date: 2021-05-06 10:12:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-06 10:24:07
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
    {
        let encode = vec![1, 2, 3];
        let first = 1;
        assert_eq!(decode(encode, first), vec![1, 0, 2, 1]);
    }
    {
        let encode = vec![6, 2, 7, 3];
        let first = 4;
        assert_eq!(decode(encode, first), vec![4, 2, 0, 7, 4]);
    }
}
