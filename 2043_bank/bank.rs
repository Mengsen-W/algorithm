/*
 * @Date: 2022-03-18 00:17:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-18 00:18:33
 * @FilePath: /algorithm/2043_bank/bank.rs
 */

struct Bank {
    money: Vec<i64>,
    total: i32,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank {
            money: balance.clone(),
            total: balance.len() as i32,
        }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 > self.total || account2 > self.total {
            return false;
        }
        if self.money[account1 as usize - 1] < money {
            return false;
        }
        self.money[account1 as usize - 1] -= money;
        self.money[account2 as usize - 1] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account > self.total {
            return false;
        }
        self.money[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account > self.total || self.money[account as usize - 1] < money {
            return false;
        }
        self.money[account as usize - 1] -= money;
        true
    }
}

fn main() {
    let mut b = Bank::new(vec![10, 100, 20, 50, 30]);
    assert_eq!(b.withdrwa(3, 10), true);
    assert_eq!(b.transfer(5, 1, 10), true);
    assert_eq!(b.deposit(5, 20), true);
    assert_eq!(b.transfer(3, 4, 15), false);
    assert_eq!(b.withdraw(10, 50), false);
}
