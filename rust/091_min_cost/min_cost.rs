/*
 * @Date: 2022-06-25
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-25
 * @FilePath: /algorithm/091_min_cost/min_cost.rs
 */

pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut dp0 = vec![0, 0, 0];
    let mut dp1 = dp0.clone();
    for c in costs {
        dp1[0] = c[0] + i32::min(dp0[1], dp0[2]);
        dp1[1] = c[1] + i32::min(dp0[0], dp0[2]);
        dp1[2] = c[2] + i32::min(dp0[0], dp0[1]);
        let t = dp0;
        dp0 = dp1;
        dp1 = t;
    }
    dp0.into_iter().min().unwrap()
}

fn main() {
    assert_eq!(
        min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
        10
    );

    assert_eq!(min_cost(vec![vec![7, 6, 2]]), 2);
}
