// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type ATM struct {
	cnt   []int64 // 每张钞票剩余数量
	value []int64 // 每张钞票面额
}

func Constructor() ATM {
	return ATM{
		cnt: make([]int64, 5),
		value: []int64{
			20, 50, 100, 200, 500,
		},
	}
}

func (this *ATM) Deposit(banknotesCount []int) {
	for i := 0; i < 5; i++ {
		this.cnt[i] += int64(banknotesCount[i])
	}
}

func (this *ATM) Withdraw(amount int) []int {
	res := make([]int, 5)
	// 模拟尝试取出钞票的过程
	for i := 4; i >= 0; i-- {
		res[i] = int(min(this.cnt[i], int64(amount)/this.value[i]))
		amount -= res[i] * int(this.value[i])
	}
	if amount > 0 {
		// 无法完成该操作
		return []int{-1}
	}
	// 可以完成该操作
	for i := 0; i < 5; i++ {
		this.cnt[i] -= int64(res[i])
	}
	return res
}

func main() {
	atm := Constructor()
	atm.Deposit([]int{0, 0, 1, 2, 1}) // 存入 1 张 $100 ，2 张 $200 和 1 张 $500 的钞票。
	assert.Equal(&testing.T{}, atm.Withdraw(600), []int{0, 0, 1, 0, 1})
	atm.Deposit([]int{0, 1, 0, 1, 1})
	assert.Equal(&testing.T{}, atm.Withdraw(600), []int{-1})
	assert.Equal(&testing.T{}, atm.Withdraw(550), []int{0, 1, 0, 0, 1})
}
