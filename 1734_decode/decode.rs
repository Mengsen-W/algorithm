/*
 * @Date: 2021-05-11 08:31:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-11 09:04:34
 */

fn decode(mut encoded: Vec<i32>) -> Vec<i32> {
    let n = encoded.len() + 1;
    let mut s = encoded[1..]
        .chunks(2)
        .fold((2..=n as i32).fold(1, |s, x| s ^ x), |s, x| s ^ x[0]);
    encoded.push(0);
    encoded.iter_mut().for_each(|x| {
        let t = *x;
        *x = s;
        s ^= t
    });
    //这里使用std::mem::replace或许会更好
    encoded
}

fn main() {
    {
        let encoded = vec![3, 1];
        assert_eq!(decode(encoded), vec![1, 2, 3]);
    }
    {
        let encoded = vec![6, 5, 4, 6];
        assert_eq!(decode(encoded), vec![2, 4, 1, 5, 3]);
    }
}
