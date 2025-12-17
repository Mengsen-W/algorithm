// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumProfit(prices []int, k int) int64 {
	n := len(prices)
	dp := make([][3]int64, k+1)
	// 初始化第 0 天的状态
	for j := 1; j <= k; j++ {
		dp[j][1] = int64(-prices[0])
		dp[j][2] = int64(prices[0])
	}

	for i := 1; i < n; i++ {
		for j := k; j > 0; j-- {
			dp[j][0] = max(dp[j][0], max(dp[j][1]+int64(prices[i]), dp[j][2]-int64(prices[i])))
			dp[j][1] = max(dp[j][1], dp[j-1][0]-int64(prices[i]))
			dp[j][2] = max(dp[j][2], dp[j-1][0]+int64(prices[i]))
		}
	}

	return dp[k][0]
}

func main() {
	tests := []struct {
		prices []int
		k      int
		ans    int64
	}{
		{[]int{1, 7, 9, 8, 2}, 2, 14},
		{[]int{12, 16, 19, 19, 8, 1, 19, 13, 9}, 3, 36},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, maximumProfit(test.prices, test.k), test.ans, index)
	}
}
