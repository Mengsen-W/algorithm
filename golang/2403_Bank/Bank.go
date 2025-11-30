// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

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
	blance := []int64{10, 100, 20, 50, 30}
	bank := Constructor(blance)
	assert.Equal(&testing.T{}, bank.Withdraw(3, 10), true)
	assert.Equal(&testing.T{}, bank.Transfer(5, 1, 20), true)
	assert.Equal(&testing.T{}, bank.Deposit(5, 20), true)
	assert.Equal(&testing.T{}, bank.Transfer(3, 4, 15), false)
	assert.Equal(&testing.T{}, bank.Withdraw(10, 50), false)
}
