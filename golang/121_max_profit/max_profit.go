/*
 * @Date: 2023-10-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-01
 * @FilePath: /algorithm/golang/121_max_profit/max_profit.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(prices []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	inf := int(1e9)
	minPrice := inf
	maxProfit := 0
	for _, price := range prices {
		maxProfit = max(maxProfit, price-minPrice)
		minPrice = min(minPrice, price)
	}
	return maxProfit
}

func main() {
	tests := []struct {
		prices []int
		ans    int
	}{
		{[]int{7, 1, 5, 3, 6, 4}, 5},
		{[]int{7, 6, 4, 3, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.prices), index)
	}
}
