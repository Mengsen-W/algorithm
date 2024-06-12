struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}

fn main() {
    let tests = vec![(9, 90), (15, 80)];

    for (purchase_amount, ans) in tests {
        assert_eq!(
            Solution::account_balance_after_purchase(purchase_amount),
            ans
        );
    }
}
