/*
 * @Date: 2021-05-12 08:57:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-12 09:19:11
 */

fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut r = 0;
    arr.push(0);
    arr.iter_mut().for_each(|x| r ^= std::mem::replace(x, r));
    queries
        .into_iter()
        .map(|x| arr[(x[1] + 1) as usize] ^ arr[x[0] as usize])
        .collect()
}

fn main() {
    {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        assert_eq!(xor_queries(arr, queries), vec![2, 7, 14, 8]);
    }
    {
        let arr = vec![4, 8, 2, 10];
        let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 0]];
        assert_eq!(xor_queries(arr, queries), vec![8, 0, 4, 4]);
    }
}
