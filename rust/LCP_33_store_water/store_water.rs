/*
 * @Date: 2023-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-21
 * @FilePath: /algorithm/rust/LCP_33_store_water/store_water.rs
 */

pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
    let n = bucket.len();

    let &max_pull = vat.iter().max().unwrap();

    (1..=max_pull)
        .map(|pull| {
            (0..n).fold(pull, |t, i| {
                let least = (vat[i] as f64 / pull as f64).ceil() as i32;
                t + std::cmp::max(0, least - bucket[i])
            })
        })
        .min()
        .unwrap_or(0)
}

fn main() {
    {
        let bucket = vec![1, 3];
        let vat = vec![6, 8];
        let ans = 4;
        assert_eq!(store_water(bucket, vat), ans);
    }

    {
        let bucket = vec![9, 0, 1];
        let vat = vec![0, 2, 2];
        let ans = 3;
        assert_eq!(store_water(bucket, vat), ans);
    }
}
