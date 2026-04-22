/*
 * @Date: 2023-10-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-04
 * @FilePath: /algorithm/rust/188_max_profit/max_profit.rs
 */

struct Solution;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut buy = vec![-prices[0]; k as usize];
        let mut sale = vec![0; k as usize];
        prices.iter().skip(1).for_each(|&x| {
            buy[0] = buy[0].max(-x);
            sale[0] = sale[0].max(buy[0] + x);
            for i in 1..k as usize {
                buy[i] = buy[i].max(sale[i - 1] - x);
                sale[i] = sale[i].max(buy[i] + x);
            }
        });
        sale[k as usize - 1]
    }
}

fn main() {
    let tests = vec![(2, vec![2, 4, 1], 2), (2, vec![3, 2, 6, 5, 0, 3], 7)];

    for (k, prices, ans) in tests {
        assert_eq!(Solution::max_profit(k, prices), ans);
    }
}
