/*
 * @Date: 2023-10-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-05
 * @FilePath: /algorithm/golang/309_max_profit/max_profit.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(prices []int) int {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	if len(prices) == 0 {
		return 0
	}
	n := len(prices)
	f0, f1, f2 := -prices[0], 0, 0
	for i := 1; i < n; i++ {
		newf0, newf1, newf2 := max(f0, f2-prices[i]), f0+prices[i], max(f1, f2)
		f0, f1, f2 = newf0, newf1, newf2
	}
	return max(f1, f2)
}

func main() {
	tests := []struct {
		prices []int
		ans    int
	}{
		{[]int{1, 2, 3, 0, 2}, 3},
		{[]int{1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.prices), index)
	}
}
