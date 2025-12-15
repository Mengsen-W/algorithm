struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let n = prices.len();
        let mut res: i64 = 1; // 平滑下降阶段的总数，初值为 dp[0]
        let mut prev: i32 = 1; // 上一个元素为结尾的平滑下降阶段的总数，初值为 dp[0]
                               // 从 1 开始遍历数组，按照递推式更新 prev 以及总数 res
        for i in 1..n {
            if prices[i] == prices[i - 1] - 1 {
                prev += 1;
            } else {
                prev = 1;
            }
            res += prev as i64;
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![3, 2, 1, 4], 7), (vec![8, 6, 7, 7], 4), (vec![1], 1)];

    for (prices, expected) in tests {
        assert_eq!(Solution::get_descent_periods(prices), expected);
    }
}
