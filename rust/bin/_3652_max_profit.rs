struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let mut profit_sum = vec![0i64; n + 1];
        let mut price_sum = vec![0i64; n + 1];
        for i in 0..n {
            profit_sum[i + 1] = profit_sum[i] + prices[i] as i64 * strategy[i] as i64;
            price_sum[i + 1] = price_sum[i] + prices[i] as i64;
        }
        let mut res = profit_sum[n];
        for i in (k - 1) as usize..n {
            let left_profit = profit_sum[i - (k as usize) + 1];
            let right_profit = profit_sum[n] - profit_sum[i + 1];
            let change_profit = price_sum[i + 1] - price_sum[i - (k as usize) / 2 + 1];
            res = res.max(left_profit + change_profit + right_profit);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 8], vec![-1, 0, 1], 2, 10),
        (vec![5, 4, 3], vec![1, 1, 0], 2, 9),
    ];

    for (test, (prices, strategy, k, expected)) in tests.into_iter().enumerate() {
        assert_eq!(
            Solution::max_profit(prices, strategy, k),
            expected,
            "Test {} failed",
            test
        );
    }
}
