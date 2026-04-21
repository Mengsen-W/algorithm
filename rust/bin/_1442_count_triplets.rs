/*
 * @Date: 2021-05-18 08:23:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-18 08:47:25
 */

fn count_triplets(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = arr.len();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut total: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    let mut s = 0;
    for k in 0..n as i32 {
        let val = arr.get(k as usize).unwrap();
        if cnt.contains_key(&(s ^ val)) {
            ans += cnt.get(&(s ^ val)).unwrap() * k - total.get(&(s ^ val)).unwrap();
        }
        let counter = cnt.entry(s).or_insert(0);
        *counter += 1;
        let counter = total.entry(s).or_insert(0);
        *counter += k;
        s ^= val;
    }
    return ans;
}

fn main() {
    assert_eq!(count_triplets(vec![2, 3, 1, 6, 7]), 4);
    assert_eq!(count_triplets(vec![1, 1, 1, 1, 1]), 10);
    assert_eq!(count_triplets(vec![2, 3]), 0);
    assert_eq!(count_triplets(vec![1, 3, 5, 7, 9]), 3);
    assert_eq!(count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]), 8);
}
