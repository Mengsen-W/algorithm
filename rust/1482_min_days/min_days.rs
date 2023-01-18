/*
 * @Date: 2021-05-09 08:52:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-09 10:03:16
 */

// pub fn Judge(bloom_day: &Vec<i32>, k: i32, target: i32) -> i32 {
//     let (mut bunches, mut flowers) = (0, 0);
//     for i in 0..bloom_day.len() {
//         match bloom_day[i].cmp(&target) {
//             std::cmp::Ordering::Greater => flowers = 0,
//             _ => flowers += 1,
//         }
//         if flowers == k {
//             bunches += 1;
//             flowers = 0;
//         }
//     }
//     return bunches as i32;
// }
fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let judge = |bloom_day: &Vec<i32>, k: i32, target: i32| -> i32 {
        let (mut bunches, mut flowers) = (0, 0);
        for i in 0..bloom_day.len() {
            match bloom_day[i].cmp(&target) {
                std::cmp::Ordering::Greater => flowers = 0,
                _ => flowers += 1,
            }
            if flowers == k {
                bunches += 1;
                flowers = 0;
            }
        }
        return bunches as i32;
    };
    let len = bloom_day.len() as i32;
    let (mut left, mut right);
    left = 1;
    match bloom_day.iter().max() {
        Some(&j) => right = j,
        None => right = 0,
    }
    if len < m * k {
        return -1;
    } else if len == m * k {
        return right;
    }
    while left < right {
        let mid = (left + right) >> 1;
        match judge(&bloom_day, k, mid).cmp(&m) {
            std::cmp::Ordering::Less => left = mid + 1,
            _ => right = mid,
        }
    }
    return left;
}

fn main() {
    {
        let bloom_day = vec![1, 10, 3, 10, 2];
        assert_eq!(min_days(bloom_day, 3, 1), 3);
    }
    {
        let bloom_day = vec![1, 10, 3, 10, 2];
        assert_eq!(min_days(bloom_day, 3, 2), -1);
    }
    {
        let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
        assert_eq!(min_days(bloom_day, 2, 3), 12);
    }
    {
        let bloom_day = vec![1000000000, 1000000000];
        assert_eq!(min_days(bloom_day, 1, 1), 1000000000);
    }
    {
        let bloom_day = vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6];
        assert_eq!(min_days(bloom_day, 4, 2), 9);
    }
}
