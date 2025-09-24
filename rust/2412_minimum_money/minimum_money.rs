struct Solution;

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut total_lose: i64 = 0;
        let mut res: i32 = 0;

        for t in transactions {
            let cost = t[0];
            let cashback = t[1];
            total_lose += std::cmp::max(cost - cashback, 0) as i64;
            res = std::cmp::max(res, std::cmp::min(cost, cashback));
        }
        total_lose + res as i64
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 1], vec![5, 0], vec![4, 2]], 10),
        (vec![vec![3, 0], vec![0, 3]], 3),
    ];

    for (transactions, ans) in tests {
        assert_eq!(Solution::minimum_money(transactions), ans);
    }
}
