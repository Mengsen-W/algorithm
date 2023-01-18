/*
 * @Date: 2022-03-18 00:17:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-18 00:27:12
 * @FilePath: /algorithm/2043_bank/bank.go
 */

package main

type Bank []int64

func Constructor(balance []int64) Bank {
	return balance
}

func (b Bank) Transfer(account1, account2 int, money int64) bool {
	if account1 > len(b) || account2 > len(b) || b[account1-1] < money {
		return false
	}
	b[account1-1] -= money
	b[account2-1] += money
	return true
}

func (b Bank) Deposit(account int, money int64) bool {
	if account > len(b) {
		return false
	}
	b[account-1] += money
	return true
}

func (b Bank) Withdraw(account int, money int64) bool {
	if account > len(b) || b[account-1] < money {
		return false
	}
	b[account-1] -= money
	return true
}

func main() {
	assert := func(a, b bool) {
		if a != b {
			panic("Not Passed")
		}
	}

	b := Constructor([]int64{10, 100, 20, 50, 30})
	assert(b.Withdraw(3, 10), true)
	assert(b.Transfer(5, 1, 20), true)
	assert(b.Deposit(5, 20), true)
	assert(b.Transfer(3, 4, 15), false)
	assert(b.Withdraw(10, 50), false)
}
