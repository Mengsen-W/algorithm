/*
 * @Date: 2023-10-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-04
 * @FilePath: /algorithm/golang/188_max_profit/max_profit.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(k int, prices []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a ...int) int {
		res := a[0]
		for _, v := range a[1:] {
			if v > res {
				res = v
			}
		}
		return res
	}
	n := len(prices)
	if n == 0 {
		return 0
	}

	k = min(k, n/2)
	buy := make([]int, k+1)
	sell := make([]int, k+1)
	buy[0] = -prices[0]
	for i := 1; i <= k; i++ {
		buy[i] = math.MinInt64 / 2
		sell[i] = math.MinInt64 / 2
	}

	for i := 1; i < n; i++ {
		buy[0] = max(buy[0], sell[0]-prices[i])
		for j := 1; j <= k; j++ {
			buy[j] = max(buy[j], sell[j]-prices[i])
			sell[j] = max(sell[j], buy[j-1]+prices[i])
		}
	}
	return max(sell...)
}

func main() {
	tests := []struct {
		k      int
		prices []int
		ans    int
	}{
		{2, []int{2, 4, 1}, 2},
		{2, []int{3, 2, 6, 5, 0, 3}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.k, test.prices), index)
	}
}
