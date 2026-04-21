/*
 * @Date: 2023-06-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-01
 * @FilePath: /algorithm/rust/2517_maximum_tastiness/maximum_tastiness.rs
 */

pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
    fn check(price: &Vec<i32>, k: i32, tastiness: i32) -> bool {
        let mut cnt = 0;
        let mut prev = std::i32::MIN >> 1;
        for p in price.iter() {
            if *p - prev >= tastiness {
                cnt += 1;
                prev = *p;
            }
        }
        cnt >= k
    }

    let n = price.len();
    let mut sorted_price = price.clone();
    sorted_price.sort();
    let mut left = 0;
    let mut right = sorted_price[n - 1] - sorted_price[0];

    while left < right {
        let mid = (left + right + 1) >> 1;
        if check(&sorted_price, k, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    return left;
}

fn main() {
    {
        let price = vec![13, 5, 1, 8, 21, 2];
        let k = 3;
        let ans = 8;
        assert_eq!(maximum_tastiness(price, k), ans);
    }

    {
        let price = vec![1, 3, 1];
        let k = 2;
        let ans = 2;
        assert_eq!(maximum_tastiness(price, k), ans);
    }

    {
        let price = vec![7, 7, 7, 7];
        let k = 2;
        let ans = 0;
        assert_eq!(maximum_tastiness(price, k), ans);
    }
}
