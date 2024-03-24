/*
 * @Date: 2024-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-24
 * @FilePath: /algorithm/golang/322_coin_change/coin_change.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func coinChange(coins []int, amount int) int {
	Max := amount + 1
	dp := make([]int, amount+1)
	for i := range dp {
		dp[i] = Max
	}
	dp[0] = 0
	for i := 1; i <= amount; i++ {
		for j := 0; j < len(coins); j++ {
			if coins[j] <= i {
				dp[i] = int(math.Min(float64(dp[i]), float64(dp[i-coins[j]]+1)))
			}
		}
	}
	if dp[amount] > amount {
		return -1
	}
	return dp[amount]
}

func main() {
	tests := []struct {
		coins  []int
		amount int
		ans    int
	}{
		{[]int{1, 2, 5}, 11, 3},
		{[]int{2}, 3, -1},
		{[]int{1}, 0, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, coinChange(test.coins, test.amount), index)
	}
}
