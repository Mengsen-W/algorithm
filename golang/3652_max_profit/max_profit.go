// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(prices []int, strategy []int, k int) int64 {
	n := len(prices)
	profitSum := make([]int64, n+1)
	priceSum := make([]int64, n+1)
	for i := 0; i < n; i++ {
		profitSum[i+1] = profitSum[i] + int64(prices[i])*int64(strategy[i])
		priceSum[i+1] = priceSum[i] + int64(prices[i])
	}
	res := profitSum[n]
	for i := k - 1; i < n; i++ {
		leftProfit := profitSum[i-k+1]
		rightProfit := profitSum[n] - profitSum[i+1]
		changeProfit := priceSum[i+1] - priceSum[i-k/2+1]
		res = max(res, leftProfit+changeProfit+rightProfit)
	}
	return res
}

func main() {
	tests := []struct {
		prices   []int
		strategy []int
		k        int
		ans      int64
	}{
		{[]int{4, 2, 8}, []int{-1, 0, 1}, 2, 10},
		{[]int{5, 4, 3}, []int{1, 1, 0}, 2, 9},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.prices, test.strategy, test.k), index)
	}
}
