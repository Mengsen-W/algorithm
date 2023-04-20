/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/rust/1187_make_array_increasing/make_array_increasing.rs
 */

pub fn make_array_increasing(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    let mut MAX = 1000000001;
    arr2.sort_unstable();
    arr2.dedup();
    arr1.push(MAX);
    arr1.insert(0, -1);

    let mut dp = vec![MAX; arr1.len()];
    dp[0] = 0;

    for i in 1..arr1.len() {
        let j = match arr2.binary_search(&arr1[i]) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        for k in 1..=j.min(i - 1) {
            if arr1[i - k - 1] < arr2[j - k] {
                dp[i] = dp[i].min(dp[i - k - 1] + k as i32);
            }
        }

        if arr1[i - 1] < arr1[i] {
            dp[i] = dp[i].min(dp[i - 1]);
        }
    }

    let res = dp[arr1.len() - 1];

    if res >= MAX {
        -1
    } else {
        res
    }
}

fn main() {
    {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![1, 3, 2, 4];
        let ans = 1;
        assert_eq!(make_array_increasing(arr1, arr2), ans);
    }

    {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![4, 3, 1];
        let ans = 2;
        assert_eq!(make_array_increasing(arr1, arr2), ans);
    }

    {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![1, 6, 3, 3];
        let ans = -1;
        assert_eq!(make_array_increasing(arr1, arr2), ans);
    }
}
