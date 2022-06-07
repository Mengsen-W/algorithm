/*
 * @Date: 2022-06-07 09:54:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-07 10:40:00
 * @FilePath: /algorithm/875_min_eating_speed/min_eating_speed.rs
 */

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut lo = 1;
    let mut hi = *piles.iter().max().unwrap();
    let mut ans = i32::MAX;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let tmp = piles
            .iter()
            .fold(0, |acc, &pile| acc + (pile + mid - 1) / mid);

        if tmp <= h {
            ans = ans.min(mid);
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
