/*
 * @Date: 2022-09-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-01
 * @FilePath: /algorithm/1475_final_prices/final_prices.rs
 */

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut res = vec![0; prices.len()];
    let n = prices.len();
    for i in (0..n).rev() {
        while let Some(&v) = stack.last() {
            if v > prices[i] {
                stack.pop();
            } else {
                res[i] = prices[i] - v;
                break;
            }
        }
        if stack.is_empty() {
            res[i] = prices[i];
        }
        stack.push(prices[i]);
    }
    res
}

fn main() {
    {
        let prices = vec![8, 4, 6, 2, 3];
        let ans = vec![4, 2, 4, 2, 3];
        assert_eq!(final_prices(prices), ans);
    }

    {
        let prices = vec![1, 2, 3, 4, 5];
        let ans = vec![1, 2, 3, 4, 5];
        assert_eq!(final_prices(prices), ans);
    }

    {
        let prices = vec![10, 1, 1, 6];
        let ans = vec![9, 0, 1, 6];
        assert_eq!(final_prices(prices), ans);
    }
}
