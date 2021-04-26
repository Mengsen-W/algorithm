/*
 * @Date: 2021-04-26 09:53:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-26 10:39:23
 */

fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
    let mut right = weights.iter().fold(0, |acc, cur| acc + cur);
    let mut left = *weights.iter().max().unwrap();

    while left < right {
        let mid = (left + right) >> 1;

        let mut days = 1;
        let mut total = 0;

        for (_, weight) in weights.iter().enumerate() {
            if total + weight > mid {
                days += 1;
                total = 0;
            }

            total += weight;
        }

        if days <= d {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let d = 5;
        assert_eq!(ship_within_days(weights, d), 15);
    }
    {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let d = 3;
        assert_eq!(ship_within_days(weights, d), 6);
    }
    {
        let weights = vec![1, 2, 3, 1, 1];
        let d = 4;
        assert_eq!(ship_within_days(weights, d), 3);
    }
}
