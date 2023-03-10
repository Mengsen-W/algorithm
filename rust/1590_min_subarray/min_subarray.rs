/*
 * @Date: 2023-03-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-10
 * @FilePath: /algorithm/rust/1590_min_subarray/min_subarray.rs
 */

pub fn min_subarray(muns: Vec<i32>, p: i32) -> i32 {
    use std::collections::HashMap;
    let x = (muns.iter().map(|&x| x as i64).sum::<i64>() % p as i64) as i32;
    if x == 0 {
        return 0;
    }
    let len = muns.len();
    let mut idx = HashMap::with_capacity(len);
    idx.insert(0, -1);
    let len = len as i32;
    let mut ans = len;
    let mut y = 0;
    for (i, &v) in muns.iter().enumerate() {
        let i = i as i32;
        y = (y + v) % p;
        if let Some(t) = idx.get(&((y - x + p) % p)) {
            ans = ans.min(i - *t);
        }
        *idx.entry(y).or_insert(0) = i;
    }
    if ans == len {
        -1
    } else {
        ans
    }
}

fn main() {
    {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        let ans = 1;
        assert_eq!(min_subarray(nums, p), ans);
    }

    {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        let ans = 2;
        assert_eq!(min_subarray(nums, p), ans);
    }

    {
        let nums = vec![1, 2, 3];
        let p = 3;
        let ans = 0;
        assert_eq!(min_subarray(nums, p), ans);
    }

    {
        let nums = vec![1, 2, 3];
        let p = 7;
        let ans = -1;
        assert_eq!(min_subarray(nums, p), ans);
    }

    {
        let nums = vec![1000000000, 1000000000, 1000000000];
        let p = 3;
        let ans = 0;
        assert_eq!(min_subarray(nums, p), ans);
    }
}
