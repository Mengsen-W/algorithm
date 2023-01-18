/*
 * @Date: 2021-05-13 08:32:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-13 08:50:59
 */

fn num_ways(steps: i32, arr_len: i32) -> i32 {
    const _MOD: usize = 1e9 as usize + 7;
    let n = (arr_len as usize).min((steps as usize + 3) / 2);
    let mut arr = vec![0; n];
    arr[0] = 1;
    for _ in 0..steps {
        let mut tmp = vec![0; n];
        tmp[0] = arr[1];
        tmp[n - 1] = arr[n - 2];
        for i in 1..arr.len() - 1 {
            tmp[i] = (arr[i - 1] + arr[i + 1]) % _MOD;
        }
        for i in 0..arr.len() {
            arr[i] = (arr[i] + tmp[i]) % _MOD;
        }
    }
    arr[0] as i32
}

fn main() {
    assert_eq!(num_ways(3, 2), 4);
    assert_eq!(num_ways(2, 4), 2);
}
