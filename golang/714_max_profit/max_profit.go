/*
 * @Date: 2023-10-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-06
 * @FilePath: /algorithm/golang/714_max_profit/max_profit.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(prices []int, fee int) int {
	n := len(prices)
	buy := prices[0] + fee
	profit := 0
	for i := 1; i < n; i++ {
		if prices[i]+fee < buy {
			buy = prices[i] + fee
		} else if prices[i] > buy {
			profit += prices[i] - buy
			buy = prices[i]
		}
	}
	return profit
}

func main() {
	tests := []struct {
		prices []int
		fee    int
		ans    int
	}{
		{[]int{1, 3, 2, 8, 4, 9}, 2, 8},
		{[]int{1, 3, 7, 5, 10, 3}, 3, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.prices, test.fee), "case:%d", index)
	}
}
