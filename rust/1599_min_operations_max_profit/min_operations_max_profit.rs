/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-01
 * @FilePath: /algorithm/rust/1599_min_operations_max_profit/min_operations_max_profit.rs
 */

struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let (mut max_profit, mut curr_profit, mut wait_customers, mut index, mut ret) =
            (0, -1, 0, 0, -1);
        if boarding_cost * 4 < running_cost {
            return ret;
        }
        while index < customers.len() || wait_customers > 0 {
            if index < customers.len() {
                wait_customers += customers[index];
            }
            index += 1;
            let cnt = 4.min(wait_customers);
            curr_profit += cnt * boarding_cost - running_cost;
            wait_customers -= cnt;
            if curr_profit > max_profit {
                max_profit = curr_profit;
                ret = index as i32;
            }
        }
        ret
    }
}

fn main() {
    let tests = vec![
        (vec![8, 3], 5, 6, 3),
        (vec![10, 9, 6], 6, 4, 7),
        (vec![3, 4, 0, 5, 1], 1, 92, -1),
    ];

    for (customers, boarding_cost, running_cost, ans) in tests {
        assert_eq!(
            Solution::min_operations_max_profit(customers, boarding_cost, running_cost),
            ans
        );
    }
}
