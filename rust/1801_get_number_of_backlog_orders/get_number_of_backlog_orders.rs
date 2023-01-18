/*
 * @Date: 2023-01-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-02
 * @FilePath: /algorithm/1801_get_number_of_backlog_orders/get_number_of_backlog_orders.rs
 */

pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1000000007;
    use std::collections::BinaryHeap; // price  amount
    let mut buy_orders = BinaryHeap::<(i32, i32)>::new(); //
    let mut sell_orders = BinaryHeap::<(i32, i32)>::new(); // Reverse

    for order in orders {
        let (price, mut amount, order_type) = (order[0], order[1], order[2]);
        if order_type == 0 {
            while amount > 0 && !sell_orders.is_empty() && -sell_orders.peek().unwrap().0 <= price {
                let mut sell_order = sell_orders.pop().unwrap();
                let sell_amount = sell_order.1.min(amount);
                amount -= sell_amount;
                sell_order.1 -= sell_amount;
                if sell_order.1 > 0 {
                    sell_orders.push(sell_order);
                }
            }
            if amount > 0 {
                buy_orders.push((price, amount));
            }
        } else {
            while amount > 0 && !buy_orders.is_empty() && buy_orders.peek().unwrap().0 >= price {
                let mut buy_order = buy_orders.pop().unwrap();
                let buy_amount = buy_order.1.min(amount);
                amount -= buy_amount;
                buy_order.1 -= buy_amount;
                if buy_order.1 > 0 {
                    buy_orders.push(buy_order);
                }
            }
            if amount > 0 {
                sell_orders.push((-price, amount));
            }
        }
    }
    let mut total = 0;
    for iter in sell_orders.iter() {
        total = (iter.1 + total) % MOD;
    }
    for iter in buy_orders.iter() {
        total = (iter.1 + total) % MOD;
    }
    return total;
}

fn main() {
    {
        let orders = vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0],
        ];
        let ans = 6;
        assert_eq!(get_number_of_backlog_orders(orders), ans);
    }

    {
        let orders = vec![
            vec![7, 1000000000, 1],
            vec![15, 3, 0],
            vec![5, 999999995, 0],
            vec![5, 1, 1],
        ];
        let ans = 999999984;
        assert_eq!(get_number_of_backlog_orders(orders), ans);
    }
}
