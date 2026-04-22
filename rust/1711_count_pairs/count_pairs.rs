/*
 * @Date: 2021-07-07 08:54:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-07 09:24:12
 */

fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    static MOD: i32 = 1_000_000_000 + 7;
    let max_val = deliciousness.iter().max().unwrap();
    let max_sum = max_val * 2;
    let mut pairs = 0;
    let mut mp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    deliciousness.iter().for_each(|&val| {
        let mut sum = 1;
        while sum <= max_sum {
            let count;
            match mp.get(&(sum - val)) {
                Some(&v) => count = v,
                None => count = 0,
            }
            pairs = (pairs + count) % MOD;
            sum <<= 1;
        }
        *mp.entry(val).or_insert(0) += 1;
    });
    pairs
}

fn main() {
    {
        let deliciousness = vec![1, 3, 5, 7, 9];
        assert_eq!(count_pairs(deliciousness), 4);
    }
    {
        let deliciousness = vec![1, 1, 1, 3, 3, 3, 7];
        assert_eq!(count_pairs(deliciousness), 15);
    }
}
