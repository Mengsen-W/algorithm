/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-05
 * @FilePath: /algorithm/rust/1599_min_operations_max_profit/min_operations_max_profit.rs
 */

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

fn main() {
    {
        let customers = vec![8, 3];
        let boarding_cost = 5;
        let running_cost = 6;
        let ans = 3;
        assert_eq!(
            min_operations_max_profit(customers, boarding_cost, running_cost),
            ans
        );
    }

    {
        let customers = vec![10, 9, 6];
        let boarding_cost = 6;
        let running_cost = 4;
        let ans = 7;
        assert_eq!(
            min_operations_max_profit(customers, boarding_cost, running_cost),
            ans
        );
    }

    {
        let customers = vec![3, 4, 0, 5, 1];
        let boarding_cost = 1;
        let running_cost = 92;
        let ans = -1;
        assert_eq!(
            min_operations_max_profit(customers, boarding_cost, running_cost),
            ans
        );
    }
}
